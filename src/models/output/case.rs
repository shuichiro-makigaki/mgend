use crate::format::turtle::ToTurtle;
use crate::models::context::Contexts;
use crate::models::input::{
    AlleleOrigin, ClinicalSignificance, CodeType, ConditionIDType, DataOrigin, DiseaseArea1,
    DiseaseArea2, Record,
};
use crate::models::name_space::{
    NameSpace, NameSpaces, MED2RDF, MGEND_CASE, MGEND_DISEASE, MGEND_ONTOLOGY, MGEND_SUBMISSION,
    MGEND_VARIANT, OBO, OLO, RDF, RDFS, SIO,
};
use crate::models::output::disease::Disease;
use crate::models::output::submission::Submission;
use crate::models::output::variant::Variant;
use crate::models::output::XRef;
use crate::models::regex;
use serde::Serialize;
use serde_json::{json, Value};
use std::io;
use std::io::Write;

const SO_SOMATIC_VARIANT: &'static str = "SO_0001777";
const SO_GERMLINE_VARIANT: &'static str = "SO_0001778";

const SEX_LABEL_MALE: &'static str = "male";
const SEX_LABEL_FEMALE: &'static str = "female";
const SEX_LABEL_MIXED_GENDER: &'static str = "mixed gender";
const SEX_LABEL_UNKNOWN: &'static str = "unknown";
const SEX_LABEL_NOT_PROVIDED: &'static str = "not provided";

#[derive(Debug, Serialize)]
pub struct Histogram {
    #[serde(rename(serialize = "type"))]
    typ: Vec<String>,
    length: usize,
    age_type: String,
    age_unit: String,
    slot: Vec<Slot>,
}

impl Histogram {
    pub fn new(values: Vec<i32>, age_type: &str, age_unit: &str) -> Self {
        let typ = vec![
            format!("{}:FrequencyDistribution", MED2RDF.prefix),
            format!("{}:OrderedList", OLO.prefix),
        ];

        Histogram {
            typ,
            length: values.len(),
            age_type: age_type.to_string(),
            age_unit: age_unit.to_string(),
            slot: values
                .iter()
                .enumerate()
                .map(|(i, &v)| Slot {
                    index: i + 1,
                    item: Item {
                        typ: format!("{}:Bin{:0>2}", MGEND_ONTOLOGY.prefix, i + 1),
                        frequency: v,
                    },
                })
                .collect(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Slot {
    index: usize,
    item: Item,
}

#[derive(Debug, Serialize)]
pub struct Item {
    #[serde(rename(serialize = "type"))]
    typ: String,
    frequency: i32,
}

#[derive(Debug, Serialize)]
pub struct SexCount {
    #[serde(rename(serialize = "type"))]
    typ: String,
    label: String,
    count: i32,
}

impl SexCount {
    pub fn new(label: &str, count: i32) -> Self {
        SexCount {
            typ: format!("{}:SexCount", MGEND_ONTOLOGY.prefix),
            label: label.to_string(),
            count,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Case {
    id: String,
    #[serde(rename(serialize = "type"))]
    typ: Vec<String>,
    variant: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    disease: Option<String>,
    submission: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    variant_type: Option<DataOrigin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allele_origin: Option<AlleleOrigin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disease_area: Option<DiseaseArea1>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub_disease_area: Option<DiseaseArea2>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    xref: Vec<XRef>,
    case_significance: ClinicalSignificance,
    case_count_total: i32,
    case_age_range_count: Vec<Histogram>,
    case_sex_count: Vec<SexCount>,
}

impl Case {
    pub fn id(r: &Record) -> String {
        match Disease::id(r) {
            None => format!("{}#{}", Submission::id(r), Variant::id(r)),
            Some(disease) => format!("{}#{}_{}", Submission::id(r), Variant::id(r), disease),
        }
    }

    pub fn identifier(&self) -> &String {
        &self.id
    }
}

impl<'a> From<&Record<'a>> for Case {
    fn from(value: &Record) -> Self {
        let r = value.row;

        let mut typ = vec![format!("{}:{}", MGEND_ONTOLOGY.prefix, "Case")];
        match r.allele_origin {
            Some(AlleleOrigin::Somatic) => {
                typ.push(format!("{}:{}", OBO.prefix, SO_SOMATIC_VARIANT))
            }
            Some(AlleleOrigin::Germline) => {
                typ.push(format!("{}:{}", OBO.prefix, SO_GERMLINE_VARIANT))
            }
            _ => {}
        }

        let mut xref = Vec::new();
        if let (Some(typ), Some(id)) = (&value.row.code_type, &value.row.code_value) {
            match typ {
                CodeType::ICD10 => {
                    if let Some(caps) = regex!(r"[A-Z]\d+(\.[-\d+])?").captures(id) {
                        xref.push(XRef::ICD10(caps[0].to_string()));
                    }
                }
                CodeType::SnomedCt => {
                    xref.push(XRef::SnomedCt(id.to_owned()));
                }
            }
        }
        if let (Some(typ), Some(id)) = (&value.row.condition_id_type, &value.row.condition_id_value)
        {
            match typ {
                ConditionIDType::MeSH => {
                    if let Some(caps) = regex!(r"[CD]\d+").captures(id) {
                        xref.push(XRef::MeSH(caps[0].to_string()));
                    }
                }
                ConditionIDType::MedGen => {
                    if let Some(caps) = regex!(r"^[CN]+\d{4,7}$").captures(id) {
                        xref.push(XRef::MedGenCID(caps[0].to_string()));
                    } else if let Some(caps) = regex!(r"^\d{4,7}$").captures(id) {
                        xref.push(XRef::MedGenUID(caps[0].to_string()));
                    }
                }
                ConditionIDType::OMIM => {
                    if let Some(caps) = regex!(r"^PS\d+$").captures(id) {
                        xref.push(XRef::OMIMPS(caps[0].to_string()));
                    } else if let Some(caps) = regex!(r"^\d+$").captures(id) {
                        xref.push(XRef::OMIM(caps[0].to_string()));
                    }
                }
                ConditionIDType::HPO => {
                    if let Some(caps) = regex!(r"\d+").captures(id) {
                        xref.push(XRef::HPO(caps[0].to_string()));
                    }
                }
                ConditionIDType::Orphanet => {
                    if let Some(caps) = regex!(r"\d+").captures(id) {
                        xref.push(XRef::Orphanet(caps[0].to_string()));
                    }
                }
            }
        }

        let case_age_count = Histogram::new(
            vec![
                r.age_0_9_numerator,
                r.age_10_19_numerator,
                r.age_20_29_numerator,
                r.age_30_39_numerator,
                r.age_40_49_numerator,
                r.age_50_59_numerator,
                r.age_60_69_numerator,
                r.age_70_79_numerator,
                r.age_80_89_numerator,
                r.age_90_99_numerator,
                r.age_100_numerator,
                r.age_unknown_numerator,
                r.age_not_provided_numerator,
            ],
            "Age",
            format!("{}:SIO_001013", SIO.prefix).as_str(),
        );

        let case_age_of_on_set_count = Histogram::new(
            vec![
                r.age_of_onset_0_9_numerator,
                r.age_of_onset_10_19_numerator,
                r.age_of_onset_20_29_numerator,
                r.age_of_onset_30_39_numerator,
                r.age_of_onset_40_49_numerator,
                r.age_of_onset_50_59_numerator,
                r.age_of_onset_60_69_numerator,
                r.age_of_onset_70_79_numerator,
                r.age_of_onset_80_89_numerator,
                r.age_of_onset_90_99_numerator,
                r.age_of_onset_100_numerator,
                r.age_of_onset_unknown_numerator,
                r.age_of_onset_not_provided_numerator,
            ],
            "AgeOfOnset",
            format!("{}:SIO_001013", SIO.prefix).as_str(),
        );

        let case_sex_count = vec![
            SexCount::new(SEX_LABEL_MALE, r.sex_male_numerator),
            SexCount::new(SEX_LABEL_FEMALE, r.sex_female_numerator),
            SexCount::new(SEX_LABEL_MIXED_GENDER, r.sex_mixed_gender_numerator),
            SexCount::new(SEX_LABEL_UNKNOWN, r.sex_unknown_numerator),
            SexCount::new(SEX_LABEL_NOT_PROVIDED, r.sex_not_provided_numerator),
        ];

        Case {
            id: Case::id(value),
            typ,
            variant: format!("{}:{}", MGEND_VARIANT.prefix, Variant::id(value)),
            submission: format!("{}:{}", MGEND_SUBMISSION.prefix, Submission::id(value)),
            disease: Disease::id(value).map(|x| format!("{}:{}", MGEND_DISEASE.prefix, x)),
            variant_type: r.data_origin.clone(),
            allele_origin: r.allele_origin.clone(),
            disease_area: r.disease_area_1.clone(),
            sub_disease_area: r.disease_area_2.clone(),
            xref,
            case_significance: r.clinical_significance.clone(),
            case_count_total: r.age_0_9_denominator,
            case_age_range_count: vec![case_age_count, case_age_of_on_set_count],
            case_sex_count,
        }
    }
}

impl NameSpaces for Case {
    fn namespaces() -> Vec<NameSpace> {
        vec![
            MED2RDF,
            MGEND_CASE,
            MGEND_DISEASE,
            MGEND_ONTOLOGY,
            MGEND_SUBMISSION,
            MGEND_VARIANT,
            OBO,
            OLO,
            RDF,
            RDFS,
            SIO,
        ]
    }
}

impl Contexts for Case {
    fn contexts() -> Value {
        json!({
          "@base": MGEND_CASE.prefix ,
          "id": "@id",
          "type": "@type",
          "allele_origin": "mgendo:allele_origin",
          "case_age_range_count": "mgendo:case_age_range_count",
          "age_type": "mgendo:age_type",
          "age_unit": {
            "@id": "sio:SIO_000221",
            "@type": "@id",
          },
          "case_count_total": "mgendo:case_count_total",
          "case_sex_count": "mgendo:case_sex_count",
          "case_significance": "mgendo:case_significance",
          "count": "rdf:value",
          "disease": {
            "@id": "m2r:disease",
            "@type": "@id"
          },
          "xref": {
            "@id": "rdfs:seeAlso",
            "@type": "@id"
          },
          "disease_area": "mgendo:disease_area",
          "frequency": "mgendo:frequency",
          "index": "olo:index",
          "item": "olo:item",
          "label": "rdfs:label",
          "length": "olo:length",
          "slot": "olo:slot",
          "sub_disease_area": "mgendo:sub_disease_area",
          "submission": {
            "@id": "mgendo:submission",
            "@type": "@id"
          },
          "variant": {
            "@id": "m2r:variation",
            "@type": "@id"
          },
          "variant_type": "mgendo:variant_type",
        })
    }
}

impl ToTurtle for Case {
    fn to_ttl(&self) -> io::Result<String> {
        let mut vec = Vec::new();

        write!(
            vec,
            "{}:{} a {}",
            MGEND_CASE.prefix,
            self.pname(&self.id),
            self.typ.join(", ")
        )?;
        write!(
            vec,
            " ;\n  {}:variant {}",
            MGEND_ONTOLOGY.prefix,
            self.pname(&self.variant)
        )?;
        write!(
            vec,
            " ;\n  {}:submission {}",
            MGEND_ONTOLOGY.prefix,
            self.pname(&self.submission)
        )?;
        if let Some(v) = &self.disease {
            write!(
                vec,
                " ;\n  {}:disease {}",
                MGEND_ONTOLOGY.prefix,
                self.pname(v)
            )?;
        }
        if let Some(v) = &self.variant_type {
            if let Ok(v) = serde_json::to_string(v) {
                write!(vec, " ;\n  {}:variant_type {}", MGEND_ONTOLOGY.prefix, v)?;
            }
        }
        if let Some(v) = &self.allele_origin {
            if let Ok(v) = serde_json::to_string(v) {
                write!(vec, " ;\n  {}:allele_origin {}", MGEND_ONTOLOGY.prefix, v)?;
            }
        }
        if let Some(v) = &self.disease_area {
            if let Ok(v) = serde_json::to_string(v) {
                write!(vec, " ;\n  {}:disease_area {}", MGEND_ONTOLOGY.prefix, v)?;
            }
        }
        if let Some(v) = &self.sub_disease_area {
            if let Ok(v) = serde_json::to_string(v) {
                write!(
                    vec,
                    " ;\n  {}:sub_disease_area {}",
                    MGEND_ONTOLOGY.prefix, v
                )?;
            }
        }
        if self.xref.len() > 0 {
            write!(
                vec,
                " ;\n  {}:seeAlso {}",
                RDFS.prefix,
                self.xref
                    .iter()
                    .map(|v| format!("<{}>", v))
                    .collect::<Vec<String>>()
                    .join(", ")
            )?;
        }
        if let Ok(v) = serde_json::to_string(&self.case_significance) {
            write!(
                vec,
                " ;\n  {}:case_significance {}",
                MGEND_ONTOLOGY.prefix, v
            )?;
        }
        write!(
            vec,
            " ;\n  {}:case_count_total {}",
            MGEND_ONTOLOGY.prefix, self.case_count_total
        )?;

        {
            for (i, hist) in self.case_age_range_count.iter().enumerate() {
                if i == 0 {
                    write!(
                        vec,
                        " ;\n  {}:case_age_range_count [",
                        MGEND_ONTOLOGY.prefix,
                    )?;
                } else {
                    write!(vec, " , [")?;
                }
                write!(vec, "\n    a {}", hist.typ.join(", "))?;
                write!(vec, " ;\n    {}:length {}", OLO.prefix, hist.length)?;
                write!(
                    vec,
                    " ;\n    {}:age_type {}",
                    MGEND_ONTOLOGY.prefix,
                    self.string(&hist.age_type)
                )?;
                write!(
                    vec,
                    " ;\n    {}:SIO_000221 {}",
                    SIO.prefix,
                    self.pname(&hist.age_unit)
                )?;
                for (i, x) in hist.slot.iter().enumerate() {
                    if i == 0 {
                        write!(vec, " ;\n    {}:slot [", OLO.prefix)?;
                    } else {
                        write!(vec, " , [")?;
                    }
                    write!(vec, "\n      {}:index {}", OLO.prefix, x.index)?;
                    write!(vec, " ;\n      {}:item [", OLO.prefix)?;
                    write!(vec, "\n        a {}", x.item.typ)?;
                    write!(vec, " ;\n        {}:value {}", RDF.prefix, x.item.frequency)?;
                    write!(vec, "\n      ]")?;
                    write!(vec, "\n    ]")?;
                }
                write!(vec, "\n  ]")?;
            }
        }

        {
            let hist = &self.case_sex_count;
            for (i, x) in hist.iter().enumerate() {
                if i == 0 {
                    write!(vec, " ;\n  {}:case_sex_count [", MGEND_ONTOLOGY.prefix,)?;
                } else {
                    write!(vec, " , [")?;
                }
                write!(vec, "\n    a {}", x.typ)?;
                write!(
                    vec,
                    " ;\n    {}:label {}",
                    RDFS.prefix,
                    self.string(&x.label)
                )?;
                write!(vec, " ;\n    {}:value {}", RDF.prefix, x.count)?;

                write!(vec, "\n  ]")?;
            }
        }

        writeln!(vec, " .")?;

        String::from_utf8(vec)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("{}", e)))
    }
}
