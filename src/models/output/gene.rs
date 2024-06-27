use crate::format::turtle::ToTurtle;
use crate::models::context::Contexts;
use crate::models::hgnc::HGNC;
use crate::models::input::Record;
use crate::models::name_space::{NameSpace, NameSpaces, HGNC, MED2RDF, MGEND_GENE, RDF, RDFS};
use once_cell::sync::OnceCell;
use serde::Serialize;
use serde_json::{json, Value};
use std::io;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Serialize)]
pub struct Gene {
    id: String,
    #[serde(rename(serialize = "type"))]
    typ: String,
    label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    xref: Option<String>,
}

impl Gene {
    pub fn parse<'a>(record: &Record<'a>) -> Option<Vec<&'a str>> {
        record
            .row
            .gene
            .as_ref()
            .map(|x| x.split(",").map(|x| x.trim()).collect())
    }

    pub fn hgnc<P: AsRef<Path>>(hgnc_path: P) -> &'static HGNC {
        static CELL: OnceCell<HGNC> = OnceCell::new();
        CELL.get_or_init(|| HGNC::from_path(hgnc_path.as_ref()).unwrap())
    }

    pub fn new<T: AsRef<str>, P: AsRef<Path>>(symbol: T, hgnc: P) -> Self {
        Gene {
            id: symbol.as_ref().to_string(),
            typ: format!("{}:Gene", MED2RDF.prefix),
            label: symbol.as_ref().to_string(),
            xref: Gene::hgnc(hgnc).find(symbol).map(|x| x.to_owned()),
        }
    }
}

impl NameSpaces for Gene {
    fn namespaces() -> Vec<NameSpace> {
        vec![MED2RDF, MGEND_GENE, RDF, RDFS, HGNC]
    }
}

impl Contexts for Gene {
    fn contexts() -> Value {
        json!({
          "@base": MGEND_GENE.prefix ,
          "id": "@id",
          "type": "@type",
          "label": "rdfs:label",
          "xref": {
            "@id": "rdfs:seeAlso",
            "@type": "@id"
          },
        })
    }
}

impl ToTurtle for Gene {
    fn to_ttl(&self) -> io::Result<String> {
        let mut vec = Vec::new();

        write!(vec, "{}:{} a {}", MGEND_GENE.prefix, self.id, self.typ)?;
        write!(vec, " ;\n  rdfs:label {}", self.string(&self.label))?;
        if let Some(ref v) = self.xref {
            write!(vec, " ;\n  rdfs:seeAlso {}:{}", HGNC.prefix, v)?;
        };

        writeln!(vec, " .")?;

        String::from_utf8(vec)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{}", e)))
    }
}
