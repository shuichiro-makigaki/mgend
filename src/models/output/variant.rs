use crate::format::turtle::ToTurtle;
use crate::models::context::Contexts;
use crate::models::input::Record;
use crate::models::name_space::{
    NameSpace, NameSpaces, FALDO, GVO, HCO, MED2RDF, MGEND_CASE, MGEND_DISEASE, MGEND_GENE,
    MGEND_ONTOLOGY, MGEND_VARIANT, RDF, SKOS,
};
use crate::models::output::case::Case;
use crate::models::output::disease::Disease;
use crate::models::output::gene::Gene;
use crate::{Assembly, VCFInt};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use serde_json::{json, Value};
use std::collections::HashSet;
use std::fmt::Debug;
use std::io;
use std::io::Write;
use strum::Display;

#[derive(Debug, Serialize)]
pub struct Variant {
    id: String,
    #[serde(rename(serialize = "type"))]
    typ: VariantType,
    location: Location,
    #[serde(rename(serialize = "ref"))]
    reference: Option<String>,
    #[serde(rename(serialize = "alt"))]
    alternate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gene: Option<Vec<String>>,
    case: Vec<String>,
    disease: HashSet<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    info: Vec<Info>,
}

#[derive(Debug, Serialize)]
pub struct Info {
    label: String,
    value: String,
}

impl Variant {
    pub fn id(r: &Record) -> String {
        format!(
            "{}_{}_{}_{}_{}_{}",
            r.assembly,
            r.row.chr,
            r.row.start,
            r.row.end,
            r.row.reference.as_ref().unwrap_or(&".".to_string()),
            r.row.alternate.as_ref().unwrap_or(&".".to_string())
        )
    }

    pub fn add_case(&mut self, case: &Case) {
        self.case
            .push(format!("{}:{}", MGEND_CASE.prefix, case.identifier()))
    }

    pub fn add_disease(&mut self, disease: &Disease) {
        self.disease
            .insert(format!("{}:{}", MGEND_DISEASE.prefix, disease.identifier()));
    }
}

impl NameSpaces for Variant {
    fn namespaces() -> Vec<NameSpace> {
        vec![
            FALDO,
            GVO,
            HCO,
            SKOS,
            GVO,
            FALDO,
            RDF,
            MED2RDF,
            MGEND_ONTOLOGY,
            MGEND_VARIANT,
            MGEND_GENE,
            MGEND_DISEASE,
            MGEND_CASE,
        ]
    }
}

impl Contexts for Variant {
    fn contexts() -> Value {
        json!({
          "@base": MGEND_VARIANT.prefix ,
          "id": "@id",
          "type": "@type",
          "Del": {
            "@id": "gvo:Del",
            "@type": "@id"
          },
          "ExactPosition": {
            "@id": "faldo:ExactPosition",
            "@type": "@id"
          },
          "InBetweenPosition": {
            "@id": "faldo:InBetweenPosition",
            "@type": "@id"
          },
          "Indel": {
            "@id": "gvo:Indel",
            "@type": "@id"
          },
          "Ins": {
            "@id": "gvo:Ins",
            "@type": "@id"
          },
          "MNV": {
            "@id": "gvo:MNV",
            "@type": "@id"
          },
          "RangePosition": {
            "@id": "faldo:RangePosition",
            "@type": "@id"
          },
          "SNV": {
            "@id": "gvo:SNV",
            "@type": "@id"
          },
          "after": "faldo:after",
          "alt": "gvo:alt",
          "before": "faldo:before",
          "begin": "faldo:begin",
          "case": {
            "@id": "mgendo:case",
            "@type": "@id"
          },
          "disease": {
            "@id": "m2r:disease",
            "@type": "@id"
          },
          "end": "faldo:end",
          "gene": {
            "@id": "m2r:gene",
            "@type": "@id"
          },
          "info": "gvo:info",
          "label": "rdfs:label",
          "location": "faldo:location",
          "note": "skos:note",
          "position": "faldo:position",
          "ref": "gvo:ref",
          "reference": {
            "@id": "faldo:reference",
            "@type": "@id"
          },
          "value": "rdf:value",
        })
    }
}

impl ToTurtle for Variant {
    fn to_ttl(&self) -> io::Result<String> {
        let mut vec = Vec::new();

        write!(
            vec,
            "{}:{} a {}:{}",
            MGEND_CASE.prefix,
            self.pname(&self.id),
            GVO.prefix,
            self.typ
        )?;

        match &self.location {
            Location::ExactPosition(p) => {
                write!(vec, " ;\n  {}:location [", FALDO.prefix)?;
                write!(vec, "\n    a {}:ExactPosition", FALDO.prefix)?;
                write!(vec, " ;\n    {}:position {}", FALDO.prefix, p.position)?;
                write!(
                    vec,
                    " ;\n    {}:reference {}",
                    FALDO.prefix,
                    self.pname(&p.reference)
                )?;
                write!(vec, "\n  ]")?;
            }
            Location::Region(p) => {
                write!(vec, " ;\n  {}:location [", FALDO.prefix)?;
                write!(vec, "\n    a {}:Region", FALDO.prefix)?;
                write!(vec, " ;\n    {}:begin [", FALDO.prefix)?;
                write!(vec, "\n      a {}:ExactPosition", FALDO.prefix)?;
                write!(
                    vec,
                    " ;\n      {}:position {}",
                    FALDO.prefix, p.begin.position
                )?;
                write!(
                    vec,
                    " ;\n      {}:reference {}",
                    FALDO.prefix,
                    self.pname(&p.begin.reference)
                )?;
                write!(vec, "\n    ]")?;
                write!(vec, " ;\n    {}:end [", FALDO.prefix)?;
                write!(vec, "\n      a {}:ExactPosition", FALDO.prefix)?;
                write!(
                    vec,
                    " ;\n      {}:position {}",
                    FALDO.prefix, p.end.position
                )?;
                write!(
                    vec,
                    " ;\n      {}:reference {}",
                    FALDO.prefix,
                    self.pname(&p.end.reference)
                )?;
                write!(vec, "\n    ]")?;
                write!(vec, "\n  ]")?;
            }
            Location::InBetweenPosition(p) => {
                write!(vec, " ;\n  {}:location [", FALDO.prefix)?;
                write!(vec, "\n    a {}:InBetweenPosition", FALDO.prefix)?;
                write!(vec, " ;\n    {}:after [", FALDO.prefix)?;
                write!(vec, "\n      a {}:ExactPosition", FALDO.prefix)?;
                write!(
                    vec,
                    " ;\n      {}:position {}",
                    FALDO.prefix, p.after.position
                )?;
                write!(
                    vec,
                    " ;\n      {}:reference {}",
                    FALDO.prefix,
                    self.pname(&p.after.reference)
                )?;
                write!(vec, "\n    ]")?;
                write!(vec, " ;\n    {}:before [", FALDO.prefix)?;
                write!(vec, "\n      a {}:ExactPosition", FALDO.prefix)?;
                write!(
                    vec,
                    " ;\n      {}:position {}",
                    FALDO.prefix, p.before.position
                )?;
                write!(
                    vec,
                    " ;\n      {}:reference {}",
                    FALDO.prefix,
                    self.pname(&p.before.reference)
                )?;
                write!(vec, "\n    ]")?;
                write!(vec, "\n  ]")?;
            }
        }

        if let Some(v) = &self.reference {
            write!(vec, " ;\n  {}:ref {}", GVO.prefix, self.string(v))?;
        }
        if let Some(v) = &self.alternate {
            write!(vec, " ;\n  {}:alt {}", GVO.prefix, self.string(v))?;
        }
        if let Some(v) = &self.gene {
            self.write_vec(&mut vec, v, format!("{}:gene", MED2RDF.prefix), |x| {
                self.pname(x)
            })?;
        }
        self.write_vec(
            &mut vec,
            &self.case,
            format!("{}:case", MGEND_ONTOLOGY.prefix),
            |x| self.pname(x),
        )?;

        self.write_set(
            &mut vec,
            &self.disease,
            format!("{}:disease", MED2RDF.prefix),
            |x| self.pname(x),
        )?;

        if let Some(v) = &self.note {
            self.write_vec(&mut vec, v, format!("{}:note", SKOS.prefix), |x| {
                self.string(x)
            })?;
        }

        for x in self.info.iter() {
            write!(vec, " ;\n  {}:info [", GVO.prefix,)?;
            write!(vec, "\n    {}:label {}", SKOS.prefix, self.string(&x.label))?;
            write!(
                vec,
                " ;\n    {}:value {}",
                RDF.prefix,
                self.string(&x.value)
            )?;
            write!(vec, "\n  ]")?;
        }

        writeln!(vec, " .")?;

        String::from_utf8(vec)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{}", e)))
    }
}

#[derive(Debug, Display, Serialize)]
enum VariantType {
    SNV,
    MNV,
    Ins,
    Del,
    Indel,
}

impl VariantType {
    pub fn new<S: AsRef<str> + Debug>(reference: Option<S>, alternate: Option<S>) -> Self {
        match (reference.as_ref(), alternate.as_ref()) {
            (Some(r), Some(a)) => match (r.as_ref().len(), a.as_ref().len()) {
                (1, 1) => Self::SNV,
                (r, a) if r > 1 && a > 1 && r == a => Self::MNV,
                (r, a) if r != a && (r != 0 || a != 0) => Self::Indel,
                _ => panic!(
                    "Error resolving variant type. {:?}/{:?}",
                    &reference, &alternate
                ),
            },
            (None, Some(_)) => Self::Ins,
            (Some(_), None) => Self::Del,
            _ => panic!("Error resolving variant type. {:?}/None", &reference),
        }
    }
}

impl<'a> From<&Record<'a>> for Variant {
    fn from(record: &Record) -> Self {
        let (a, r) = (record.assembly, record.row);

        let typ = VariantType::new(r.reference.as_ref(), r.alternate.as_ref());
        let location = Location::new(&typ, &r.chr, a, r.start, r.end);

        let mut info = Vec::new();
        if let Some(x) = r.rs.as_ref().and_then(|x| x.strip_prefix("rs")) {
            info.push(Info {
                label: "RS".to_string(),
                value: x.to_string(),
            })
        }

        Variant {
            id: Variant::id(record),
            typ,
            location,
            reference: r.reference.clone(),
            alternate: r.alternate.clone(),
            gene: Gene::parse(record).map(|symbols| {
                symbols
                    .iter()
                    .map(|symbol| format!("{}:{}", MGEND_GENE.prefix, symbol))
                    .collect()
            }),
            case: Vec::new(),
            disease: HashSet::new(),
            note: r
                .hgvs
                .as_ref()
                .map(|x| x.split(",").map(|x| x.trim().to_string()).collect()),
            info,
        }
    }
}

#[derive(Debug)]
enum Location {
    ExactPosition(ExactPosition),
    Region(Region),
    InBetweenPosition(InBetweenPosition),
}

impl Location {
    pub fn new<S: AsRef<str>>(
        typ: &VariantType,
        chrom: S,
        assembly: &Assembly,
        start: VCFInt,
        end: VCFInt,
    ) -> Self {
        let mut chr = chrom.as_ref().replace("chr", "");
        if chr == "M" {
            chr = "MT".to_string();
        }

        let reference = format!("{}:{}/{}", HCO.prefix, chr, assembly);

        match typ {
            VariantType::SNV => Location::ExactPosition(ExactPosition {
                position: start,
                reference,
            }),
            VariantType::MNV | VariantType::Del | VariantType::Indel => Location::Region(Region {
                begin: ExactPosition {
                    position: start,
                    reference: reference.clone(),
                },
                end: ExactPosition {
                    position: end,
                    reference,
                },
            }),
            VariantType::Ins => Location::InBetweenPosition(InBetweenPosition {
                after: ExactPosition {
                    position: start,
                    reference: reference.clone(),
                },
                before: ExactPosition {
                    position: end + 1,
                    reference,
                },
            }),
        }
    }
}

impl Serialize for Location {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Location::ExactPosition(x) => x.serialize(serializer),
            Location::Region(x) => x.serialize(serializer),
            Location::InBetweenPosition(x) => x.serialize(serializer),
        }
    }
}

#[derive(Debug)]
struct ExactPosition {
    position: VCFInt,
    reference: String,
}

const RDF_TYPE_FALDO_EXACT_POSITION: &'static str = "ExactPosition";
const RDF_TYPE_FALDO_REGION: &'static str = "Region";
const RDF_TYPE_FALDO_IN_BETWEEN_POSITION: &'static str = "InBetweenPosition";

impl Serialize for ExactPosition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("ExactPosition", 3)?;
        s.serialize_field("type", RDF_TYPE_FALDO_EXACT_POSITION)?;
        s.serialize_field("position", &self.position)?;
        s.serialize_field("reference", &self.reference)?;
        s.end()
    }
}

#[derive(Debug)]
struct Region {
    begin: ExactPosition,
    end: ExactPosition,
}

impl Serialize for Region {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("Region", 3)?;
        s.serialize_field("type", RDF_TYPE_FALDO_REGION)?;
        s.serialize_field("begin", &self.begin)?;
        s.serialize_field("end", &self.end)?;
        s.end()
    }
}

#[derive(Debug)]
struct InBetweenPosition {
    after: ExactPosition,
    before: ExactPosition,
}

impl Serialize for InBetweenPosition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("InBetweenPosition", 3)?;
        s.serialize_field("type", RDF_TYPE_FALDO_IN_BETWEEN_POSITION)?;
        s.serialize_field("after", &self.after)?;
        s.serialize_field("before", &self.before)?;
        s.end()
    }
}
