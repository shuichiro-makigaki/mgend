use crate::format::turtle::ToTurtle;
use crate::models::context::Contexts;
use crate::models::input::{DiseaseArea1, Record};
use crate::models::name_space::{
    NameSpace, NameSpaces, FOAF, MGEND_CASE, MGEND_ONTOLOGY, MGEND_SUBMISSION, OLO, ORG, PAV, RDFS,
};
use crate::models::output::case::Case;
use serde::Serialize;
use serde_json::{json, Value};
use std::io;
use std::io::Write;

#[derive(Debug, Serialize)]
pub struct Submission {
    id: String,
    #[serde(rename(serialize = "type"))]
    typ: String,
    label: String,
    submissions: Submissions,
    #[serde(skip_serializing_if = "Option::is_none")]
    disease_area: Option<DiseaseArea1>,
    case: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Submissions {
    #[serde(rename(serialize = "type"))]
    typ: Vec<String>,
    length: usize,
    submitters: Vec<Slot>,
}

#[derive(Debug, Serialize)]
pub struct Slot {
    index: usize,
    item: Submitter,
}

#[derive(Debug, Clone, Serialize)]
pub struct Submitter {
    #[serde(rename(serialize = "type"))]
    typ: String,
    name: String,
    organization: Organization,
}

#[derive(Debug, Clone, Serialize)]
pub struct Organization {
    #[serde(rename(serialize = "type"))]
    typ: String,
    label: String,
}

impl Submission {
    pub fn id(r: &Record) -> String {
        r.row.submission_id.clone()
    }

    pub fn add_case(&mut self, case: &Case) {
        self.case
            .push(format!("{}:{}", MGEND_CASE.prefix, case.identifier()))
    }
}

fn add_submitter(submitters: &mut Vec<Submitter>, name: &String, inst: &String) {
    submitters.push(Submitter {
        typ: format!("{}:Person", FOAF.prefix),
        name: name.to_string(),
        organization: Organization {
            typ: format!("{}:Organization", ORG.prefix),
            label: inst.to_string(),
        },
    })
}

impl<'a> From<&Record<'a>> for Submission {
    fn from(record: &Record) -> Self {
        let r = record.row;

        let mut submitters = Vec::new();
        if let (Some(name), Some(inst)) = (&r.submitter_name_1, &r.submitter_institute_1) {
            add_submitter(&mut submitters, name, inst);
        }
        if let (Some(name), Some(inst)) = (&r.submitter_name_2, &r.submitter_institute_2) {
            add_submitter(&mut submitters, name, inst);
        }

        Submission {
            id: Submission::id(record),
            typ: format!("{}:Submission", MGEND_ONTOLOGY.prefix),
            label: Submission::id(record),
            submissions: Submissions {
                typ: vec![
                    format!("{}:Submitters", MGEND_ONTOLOGY.prefix),
                    format!("{}:OrderList", OLO.prefix),
                ],
                length: submitters.len(),
                submitters: submitters
                    .iter()
                    .enumerate()
                    .map(|(i, x)| Slot {
                        index: i + 1,
                        item: x.clone(),
                    })
                    .collect(),
            },
            disease_area: record.row.disease_area_1.clone(),
            case: Vec::new(),
        }
    }
}

impl NameSpaces for Submission {
    fn namespaces() -> Vec<NameSpace> {
        vec![
            MGEND_CASE,
            MGEND_ONTOLOGY,
            MGEND_SUBMISSION,
            FOAF,
            OLO,
            ORG,
            PAV,
            RDFS,
        ]
    }
}

impl Contexts for Submission {
    fn contexts() -> Value {
        json!({
          "@base": MGEND_SUBMISSION.prefix ,
          "id": "@id",
          "type": "@type",
          "case": {
            "@id": "mgendo:case",
            "@type": "@id"
          },
          "disease_area": "mgendo:disease_area",
          "index": "olo:index",
          "item": "olo:item",
          "label": "rdfs:label",
          "length": "olo:length",
          "name": "foaf:name",
          "organization": "org:memberOf",
          "submissions": "pav:providedBy",
          "submitters": "olo:slot",
        })
    }
}

impl ToTurtle for Submission {
    fn to_ttl(&self) -> io::Result<String> {
        let mut vec = Vec::new();

        write!(
            vec,
            "{}:{} a {}",
            MGEND_SUBMISSION.prefix, self.id, self.typ
        )?;
        write!(vec, " ;\n  rdfs:label {}", self.string(&self.label))?;

        let submissions = &self.submissions;

        write!(vec, " ;\n  {}:providedBy [", PAV.prefix,)?;
        write!(vec, "\n    a {}", submissions.typ.join(", "))?;
        write!(vec, " ;\n    {}:length {}", OLO.prefix, submissions.length)?;
        for (i, x) in submissions.submitters.iter().enumerate() {
            if i == 0 {
                write!(vec, " ;\n    {}:slot [", OLO.prefix)?;
            } else {
                write!(vec, " , [")?;
            }

            write!(vec, "\n      {}:index {}", OLO.prefix, x.index)?;
            write!(vec, " ;\n      {}:item [", OLO.prefix)?;
            write!(vec, "\n        a {}:Person", FOAF.prefix)?;
            write!(
                vec,
                " ;\n        {}:name {}",
                FOAF.prefix,
                self.string(&x.item.name)
            )?;
            write!(vec, " ;\n        {}:memberOf [", ORG.prefix)?;
            write!(vec, "\n          a {}:Organization", ORG.prefix)?;
            write!(
                vec,
                " ;\n          {}:label {}",
                RDFS.prefix,
                self.string(&x.item.organization.label)
            )?;
            write!(vec, "\n        ]")?;
            write!(vec, "\n      ]")?;
            write!(vec, "\n    ]")?;
        }
        write!(vec, "\n  ]")?;

        if let Some(v) = &self.disease_area {
            if let Ok(v) = serde_json::to_string(v) {
                write!(vec, " ;\n  {}:disease_area {}", MGEND_ONTOLOGY.prefix, v)?;
            }
        }

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
