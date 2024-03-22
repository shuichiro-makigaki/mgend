use crate::format::turtle::ToTurtle;
use crate::models::context::Contexts;
use crate::models::input::Record;
use crate::models::name_space::{
    NameSpace, NameSpaces, MED2RDF, MGEND_CASE, MGEND_DISEASE, MGEND_ONTOLOGY, RDF, RDFS,
};
use crate::models::output::case::Case;
use serde::Serialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::io;
use std::io::Write;

#[derive(Debug, Serialize)]
pub struct Disease {
    pub id: String,
    #[serde(rename(serialize = "type"))]
    typ: String,
    label: String,
    case: Vec<String>,
}

impl Disease {
    pub fn id(record: &Record) -> Option<String> {
        match record.row.disease_name {
            None => None,
            Some(ref name) => {
                let mut hasher = Sha256::new();
                hasher.update(name);

                Some(format!("{:x}", hasher.finalize()))
            }
        }
    }

    pub fn identifier(&self) -> &String {
        &self.id
    }

    pub fn add_case(&mut self, case: &Case) {
        self.case
            .push(format!("{}:{}", MGEND_CASE.prefix, case.identifier()))
    }
}

const ERR_BLANK_DISEASE_ID: &'static str = "disease id is blank";
const ERR_BLANK_DISEASE_NAME: &'static str = "disease name is blank";

impl<'a> TryFrom<&Record<'a>> for Disease {
    type Error = &'static str;

    fn try_from(record: &Record<'a>) -> Result<Self, Self::Error> {
        let r = record.row;

        Ok(Disease {
            id: Disease::id(record).ok_or(ERR_BLANK_DISEASE_ID)?,
            typ: format!("{}:Disease", MED2RDF.prefix),
            label: r.disease_name.clone().ok_or(ERR_BLANK_DISEASE_NAME)?,
            case: Vec::new(),
        })
    }
}

impl NameSpaces for Disease {
    fn namespaces() -> Vec<NameSpace> {
        vec![
            MED2RDF,
            MGEND_ONTOLOGY,
            MGEND_CASE,
            MGEND_DISEASE,
            RDF,
            RDFS,
        ]
    }
}

impl Contexts for Disease {
    fn contexts() -> Value {
        json!({
          "@base": MGEND_DISEASE.prefix ,
          "id": "@id",
          "type": "@type",
          "case": {
            "@id": "mgendo:case",
            "@type": "@id"
          },
          "disease": {
            "@id": "mgendo:disease",
            "@type": "@id"
          },
          "gene": {
            "@id": "mgendo:gene",
            "@type": "@id"
          },
          "label": "rdfs:label",
        })
    }
}

impl ToTurtle for Disease {
    fn to_ttl(&self) -> io::Result<String> {
        let mut vec = Vec::new();

        write!(vec, "{}:{} a {}", MGEND_DISEASE.prefix, self.id, self.typ)?;
        write!(vec, " ;\n  rdfs:label {}", self.string(&self.label))?;

        self.write_vec(
            &mut vec,
            &self.case,
            format!("{}:case", MGEND_ONTOLOGY.prefix),
            |x| self.pname(x),
        )?;

        writeln!(vec, " .")?;

        String::from_utf8(vec)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{}", e)))
    }
}
