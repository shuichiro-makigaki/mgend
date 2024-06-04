use serde::{Serialize, Serializer};
use std::fmt::{Display, Formatter};

pub mod case;
pub mod disease;
pub mod gene;
pub mod submission;
pub mod variant;

#[derive(Debug)]
pub enum XRef {
    ICD10(String),
    SnomedCt(String),
    MeSH(String),
    MedGenUID(String),
    MedGenCID(String),
    OMIM(String),
    OMIMPS(String),
    HPO(String),
    Orphanet(String),
}

impl Serialize for XRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("{}", self))
    }
}

impl Display for XRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            XRef::ICD10(id) => format!("http://purl.bioontology.org/ontology/ICD10CM/{}", id),
            XRef::SnomedCt(id) => format!("http://identifiers.org/snomedct/{}", id),
            XRef::MeSH(id) => format!("http://identifiers.org/mesh/{}", id),
            XRef::MedGenUID(id) => format!("http://identifiers.org/medgen/{}", id),
            XRef::MedGenCID(id) => format!("http://www.ncbi.nlm.nih.gov/medgen/{}", id),
            XRef::OMIM(id) => format!("https://omim.org/entry/{}", id),
            XRef::OMIMPS(id) => format!("https://omim.org/phenotypicSeries/{}", id),
            XRef::HPO(id) => format!("http://purl.obolibrary.org/obo/HP_{}", id),
            XRef::Orphanet(id) => {
                format!("http://purl.obolibrary.org/obo/Orphanet_{}", id)
            }
        })
    }
}
