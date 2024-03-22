pub struct NameSpace {
    pub prefix: &'static str,
    pub uri: &'static str,
}

pub trait NameSpaces {
    fn namespaces() -> Vec<NameSpace>;
}

macro_rules! ns {
    ($name:ident, $prefix:expr, $url:expr) => {
        pub const $name: NameSpace = NameSpace {
            prefix: $prefix,
            uri: $url,
        };
    };
}

ns!(FALDO, "faldo", "http://biohackathon.org/resource/faldo#");
ns!(FOAF, "foaf", "http://xmlns.com/foaf/0.1/");
ns!(GVO, "gvo", "http://genome-variation.org/");
ns!(HCO, "hco", "http://identifiers.org/hco/");
ns!(HGNC, "hgnc", "http://identifiers.org/hgnc/");
ns!(OBO, "obo", "http://purl.obolibrary.org/obo/");
ns!(OLO, "olo", "http://purl.org/ontology/olo/core#");
ns!(ORG, "org", "https://www.w3.org/ns/org#");
ns!(PAV, "pav", "http://purl.org/pav/");
ns!(RDF, "rdf", "http://www.w3.org/1999/02/22-rdf-syntax-ns#");
ns!(RDFS, "rdfs", "http://www.w3.org/2000/01/rdf-schema#");
ns!(SIO, "sio", "http://semanticscience.org/resource/");
ns!(SKOS, "skos", "http://www.w3.org/2004/02/skos/core#");
ns!(MED2RDF, "m2r", "http://med2rdf.org/ontology/med2rdf#");
ns!(
    MGEND_ONTOLOGY,
    "mgendo",
    "http://med2rdf.org/mgend/ontology#"
);
ns!(MGEND_CASE, "mgend_case", "http://med2rdf.org/mgend/case/");
ns!(
    MGEND_DISEASE,
    "mgend_disease",
    "http://med2rdf.org/mgend/disease/"
);
ns!(MGEND_GENE, "mgend_gene", "http://med2rdf.org/mgend/gene/");
ns!(
    MGEND_SUBMISSION,
    "mgend_submission",
    "http://med2rdf.org/mgend/submission/"
);
ns!(
    MGEND_VARIANT,
    "mgend_variant",
    "http://med2rdf.org/mgend/variant/"
);
