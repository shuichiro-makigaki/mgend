mod format;
mod models;

use crate::format::jsonld::JSONLD;
use crate::format::turtle::ToTurtle;
use crate::models::context::Contexts;
use crate::models::input::Record;
use crate::models::name_space::{NameSpace, NameSpaces, MGEND_DISEASE};
use crate::models::output::disease::Disease;
use crate::models::output::gene::Gene;
use crate::models::output::submission::Submission;
use crate::models::output::variant::Variant;
use csv::ReaderBuilder;
use flate2::bufread::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use models::input::Row;
use models::output::case::Case;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use strum::{Display, EnumString, EnumVariantNames, VariantNames};

type VCFInt = i32;

#[derive(Debug, Display, EnumString, EnumVariantNames, Serialize, Clone)]
pub enum Assembly {
    GRCh37,
    GRCh38,
}

#[derive(Debug, EnumString, EnumVariantNames)]
pub enum Format {
    #[strum(serialize = "jsonld")]
    JSONLD,
    #[strum(serialize = "turtle")]
    Turtle,
}

#[derive(Debug, StructOpt)]
struct Options {
    /// Assembly
    #[structopt(long, possible_values(Assembly::VARIANTS))]
    assembly: Assembly,

    /// Output format
    #[structopt(long, possible_values(Format::VARIANTS))]
    format: Format,

    /// Path to hgnc_complete_set.txt (wget ftp://ftp.ebi.ac.uk/pub/databases/genenames/new/tsv/hgnc_complete_set.txt)
    #[structopt(long, parse(from_os_str))]
    hgnc: PathBuf,

    /// Process only one line
    #[structopt(long)]
    rehearsal: bool,

    /// Path to output directory
    #[structopt(long, parse(from_os_str))]
    directory: PathBuf,

    /// Path to input file [*.tsv | *.tsv.gz]
    #[structopt(parse(from_os_str))]
    input: PathBuf,
}

macro_rules! write_jsonld {
    ($output:expr, $filename:expr, $context:expr, $graph:expr) => {
        write_jsonld(&$output, $filename, &JSONLD::new(&$context, &$graph))
    };
}

fn main() -> io::Result<()> {
    let option = Options::from_args();

    let f = File::open(&option.input).unwrap();
    let r: Box<dyn io::Read> = match option.input.extension() {
        Some(ext) if ext == "gz" => Box::new(GzDecoder::new(BufReader::new(f))),
        _ => Box::new(BufReader::new(f)),
    };

    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b'\t')
        .from_reader(r);

    let mut cases = Vec::new();
    let mut submissions = HashMap::new();
    let mut genes = HashMap::new();
    let mut diseases = HashMap::new();
    let mut variants = HashMap::new();

    for result in reader.deserialize::<Row>() {
        let row = result?;
        let record = Record::new(&option.assembly, &row);

        let case = Case::from(&record);

        submissions
            .entry(Submission::id(&record))
            .or_insert(Submission::from(&record))
            .add_case(&case);

        if let Some(symbols) = Gene::parse(&record) {
            for symbol in symbols {
                genes
                    .entry(symbol.to_string())
                    .or_insert(Gene::new(symbol, &option.hgnc));
            }
        }

        let variant = variants
            .entry(Variant::id(&record))
            .or_insert(Variant::from(&record));

        variant.add_case(&case);

        if let Ok(disease) = Disease::try_from(&record) {
            variant.add_disease(&disease);

            diseases
                .entry(format!(
                    "{}:{}",
                    MGEND_DISEASE.prefix,
                    disease.id.to_string()
                ))
                .or_insert(disease)
                .add_case(&case);
        }

        cases.push(case);

        if option.rehearsal {
            break;
        }
    }

    match option.format {
        Format::JSONLD => {
            write_jsonld!(
                option.directory,
                "mgend_case.jsonld.gz",
                Case::contexts(),
                cases
            )?;
            write_jsonld!(
                option.directory,
                "mgend_variant.jsonld.gz",
                Variant::contexts(),
                variants.values().collect::<Vec<&Variant>>()
            )?;
            write_jsonld!(
                option.directory,
                "mgend_submission.jsonld.gz",
                Submission::contexts(),
                submissions.values().collect::<Vec<&Submission>>()
            )?;
            write_jsonld!(
                option.directory,
                "mgend_disease.jsonld.gz",
                Disease::contexts(),
                diseases.values().collect::<Vec<&Disease>>()
            )?;
            write_jsonld!(
                option.directory,
                "mgend_gene.jsonld.gz",
                Gene::contexts(),
                genes.values().collect::<Vec<&Gene>>()
            )?
        }
        Format::Turtle => {
            write_turtle(
                &option.directory,
                "mgend_case.ttl.gz",
                &Case::namespaces(),
                &cases,
            )?;
            write_turtle(
                &option.directory,
                "mgend_variant.ttl.gz",
                &Variant::namespaces(),
                variants.values(),
            )?;
            write_turtle(
                &option.directory,
                "mgend_submission.ttl.gz",
                &Submission::namespaces(),
                submissions.values(),
            )?;
            write_turtle(
                &option.directory,
                "mgend_disease.ttl.gz",
                &Disease::namespaces(),
                diseases.values(),
            )?;
            write_turtle(
                &option.directory,
                "mgend_gene.ttl.gz",
                &Gene::namespaces(),
                genes.values(),
            )?
        }
    };

    Ok(())
}

fn write_jsonld<P: AsRef<Path>, S: Serialize, T: Serialize>(
    path: P,
    filename: &str,
    obj: &JSONLD<S, T>,
) -> io::Result<()> {
    let mut out = PathBuf::from(path.as_ref());
    out.push(filename);

    let f = File::create(&out)?;
    let mut writer = GzEncoder::new(f, Compression::default());

    eprintln!("writing {:?}", &out);
    writer.write_all(serde_json::to_string_pretty(&obj).unwrap().as_bytes())?;
    writer.finish()?;

    Ok(())
}

fn write_turtle<'a, P: AsRef<Path>, T: ToTurtle + 'a, I: IntoIterator<Item = &'a T>>(
    path: P,
    filename: &str,
    namespaces: &Vec<NameSpace>,
    collection: I,
) -> io::Result<()> {
    let mut out = PathBuf::from(path.as_ref());
    out.push(filename);

    let f = File::create(&out)?;
    let mut writer = BufWriter::new(GzEncoder::new(f, Compression::default()));

    eprintln!("writing {:?}", &out);

    for ns in namespaces {
        writer.write_all(format!("@prefix {}: <{}> .\n", ns.prefix, ns.uri).as_bytes())?;
    }

    for item in collection {
        writer.write_all(b"\n")?;
        writer.write_all(item.to_ttl()?.as_bytes())?;
    }

    Ok(())
}
