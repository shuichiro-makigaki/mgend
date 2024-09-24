use crate::{Assembly, VCFInt};
use serde::{Deserialize, Serialize};

pub struct Record<'a> {
    pub assembly: &'a Assembly,
    pub row: &'a Row,
}

impl<'a> Record<'a> {
    pub fn new(assembly: &'a Assembly, row: &'a Row) -> Self {
        Record { assembly, row }
    }
}

#[derive(Debug, Deserialize)]
pub struct Row {
    #[serde(rename(deserialize = "Chr"))]
    pub chr: String,
    #[serde(rename(deserialize = "RsID"))]
    pub rs: Option<String>,
    #[serde(rename(deserialize = "Start"))]
    pub start: VCFInt,
    #[serde(rename(deserialize = "End"))]
    pub end: VCFInt,
    #[serde(rename(deserialize = "Ref"))]
    pub reference: Option<String>,
    #[serde(rename(deserialize = "Alt"))]
    pub alternate: Option<String>,
    /// Comma-separated list
    #[serde(rename(deserialize = "Gene"))]
    pub gene: Option<String>,
    #[serde(rename(deserialize = "AminoAcid"))]
    pub amino_acid: Option<String>,
    #[serde(rename(deserialize = "HGVS"))]
    pub hgvs: Option<String>,
    #[serde(rename(deserialize = "CS"))]
    pub clinical_significance: ClinicalSignificance,
    #[serde(rename(deserialize = "SexMaleNumerator"))]
    pub sex_male_numerator: i32,
    #[serde(rename(deserialize = "SexMaleDenominator"))]
    pub sex_male_denominator: i32,
    #[serde(rename(deserialize = "SexFemaleNumerator"))]
    pub sex_female_numerator: i32,
    #[serde(rename(deserialize = "SexFemaleDenominator"))]
    pub sex_female_denominator: i32,
    #[serde(rename(deserialize = "SexMixedGenderNumerator"))]
    pub sex_mixed_gender_numerator: i32,
    #[serde(rename(deserialize = "SexMixedGenderDenominator"))]
    pub sex_mixed_gender_denominator: i32,
    #[serde(rename(deserialize = "SexUnknownNumerator"))]
    pub sex_unknown_numerator: i32,
    #[serde(rename(deserialize = "SexUnknownDenominator"))]
    pub sex_unknown_denominator: i32,
    #[serde(rename(deserialize = "SexNotProvidedNumerator"))]
    pub sex_not_provided_numerator: i32,
    #[serde(rename(deserialize = "SexNotProvidedDenominator"))]
    pub sex_not_provided_denominator: i32,
    #[serde(rename(deserialize = "Age0_9Numerator"))]
    pub age_0_9_numerator: i32,
    #[serde(rename(deserialize = "Age0_9Denominator"))]
    pub age_0_9_denominator: i32,
    #[serde(rename(deserialize = "Age10_19Numerator"))]
    pub age_10_19_numerator: i32,
    #[serde(rename(deserialize = "Age10_19Denominator"))]
    pub age_10_19_denominator: i32,
    #[serde(rename(deserialize = "Age20_29Numerator"))]
    pub age_20_29_numerator: i32,
    #[serde(rename(deserialize = "Age20_29Denominator"))]
    pub age_20_29_denominator: i32,
    #[serde(rename(deserialize = "Age30_39Numerator"))]
    pub age_30_39_numerator: i32,
    #[serde(rename(deserialize = "Age30_39Denominator"))]
    pub age_30_39_denominator: i32,
    #[serde(rename(deserialize = "Age40_49Numerator"))]
    pub age_40_49_numerator: i32,
    #[serde(rename(deserialize = "Age40_49Denominator"))]
    pub age_40_49_denominator: i32,
    #[serde(rename(deserialize = "Age50_59Numerator"))]
    pub age_50_59_numerator: i32,
    #[serde(rename(deserialize = "Age50_59Denominator"))]
    pub age_50_59_denominator: i32,
    #[serde(rename(deserialize = "Age60_69Numerator"))]
    pub age_60_69_numerator: i32,
    #[serde(rename(deserialize = "Age60_69Denominator"))]
    pub age_60_69_denominator: i32,
    #[serde(rename(deserialize = "Age70_79Numerator"))]
    pub age_70_79_numerator: i32,
    #[serde(rename(deserialize = "Age70_79Denominator"))]
    pub age_70_79_denominator: i32,
    #[serde(rename(deserialize = "Age80_89Numerator"))]
    pub age_80_89_numerator: i32,
    #[serde(rename(deserialize = "Age80_89Denominator"))]
    pub age_80_89_denominator: i32,
    #[serde(rename(deserialize = "Age90_99Numerator"))]
    pub age_90_99_numerator: i32,
    #[serde(rename(deserialize = "Age90_99Denominator"))]
    pub age_90_99_denominator: i32,
    #[serde(rename(deserialize = "Age100Numerator"))]
    pub age_100_numerator: i32,
    #[serde(rename(deserialize = "Age100Denominator"))]
    pub age_100_denominator: i32,
    #[serde(rename(deserialize = "AgeUnknownNumerator"))]
    pub age_unknown_numerator: i32,
    #[serde(rename(deserialize = "AgeUnknownDenominator"))]
    pub age_unknown_denominator: i32,
    #[serde(rename(deserialize = "AgeNotProvidedNumerator"))]
    pub age_not_provided_numerator: i32,
    #[serde(rename(deserialize = "AgeNotProvidedDenominator"))]
    pub age_not_provided_denominator: i32,
    #[serde(rename(deserialize = "AgeOtherNumerator"))]
    pub age_other_numerator: i32,
    #[serde(rename(deserialize = "AgeOtherDenominator"))]
    pub age_other_denominator: i32,
    #[serde(rename(deserialize = "AgeOfOnset0_9Numerator"))]
    pub age_of_onset_0_9_numerator: i32,
    #[serde(rename(deserialize = "AgeOfOnset0_9Denominator"))]
    pub age_of_onset_0_9_denominator: i32,
    #[serde(rename(deserialize = "AgeOfOnset10_19Numerator"))]
    pub age_of_onset_10_19_numerator: i32,
    #[serde(rename(deserialize = "AgeOfOnset10_19Denominator"))]
    pub age_of_onset_10_19_denominator: i32,
    #[serde(rename(deserialize = "AgeOfOnset20_29Numerator"))]
    pub age_of_onset_20_29_numerator: i32,
    #[serde(rename(deserialize = "AgeOfOnset20_29Denominator"))]
    pub age_of_onset_20_29_denominator: i32,
    #[serde(rename(deserialize = "AgeOfOnset30_39Numerator"))]
    pub age_of_onset_30_39_numerator: i32,
    #[serde(rename(deserialize = "AgeOfOnset30_39Denominator"))]
    pub age_of_onset_30_39_denominator: i32,
    #[serde(rename(deserialize = "AgeOfOnset40_49Numerator"))]
    pub age_of_onset_40_49_numerator: i32,
    #[serde(rename(deserialize = "AgeOfOnset40_49Denominator"))]
    pub age_of_onset_40_49_denominator: i32,
    #[serde(rename(deserialize = "AgeOfOnset50_59Numerator"))]
    pub age_of_onset_50_59_numerator: i32,
    #[serde(rename(deserialize = "AgeOfOnset50_59Denominator"))]
    pub age_of_onset_50_59_denominator: i32,
    #[serde(rename(deserialize = "AgeOfOnset60_69Numerator"))]
    pub age_of_onset_60_69_numerator: i32,
    #[serde(rename(deserialize = "AgeOfOnset60_69Denominator"))]
    pub age_of_onset_60_69_denominator: i32,
    #[serde(rename(deserialize = "AgeOfOnset70_79Numerator"))]
    pub age_of_onset_70_79_numerator: i32,
    #[serde(rename(deserialize = "AgeOfOnset70_79Denominator"))]
    pub age_of_onset_70_79_denominator: i32,
    #[serde(rename(deserialize = "AgeOfOnset80_89Numerator"))]
    pub age_of_onset_80_89_numerator: i32,
    #[serde(rename(deserialize = "AgeOfOnset80_89Denominator"))]
    pub age_of_onset_80_89_denominator: i32,
    #[serde(rename(deserialize = "AgeOfOnset90_99Numerator"))]
    pub age_of_onset_90_99_numerator: i32,
    #[serde(rename(deserialize = "AgeOfOnset90_99Denominator"))]
    pub age_of_onset_90_99_denominator: i32,
    #[serde(rename(deserialize = "AgeOfOnset100Numerator"))]
    pub age_of_onset_100_numerator: i32,
    #[serde(rename(deserialize = "AgeOfOnset100Denominator"))]
    pub age_of_onset_100_denominator: i32,
    #[serde(rename(deserialize = "AgeOfOnsetUnknownNumerator"))]
    pub age_of_onset_unknown_numerator: i32,
    #[serde(rename(deserialize = "AgeOfOnsetUnknownDenominator"))]
    pub age_of_onset_unknown_denominator: i32,
    #[serde(rename(deserialize = "AgeOfOnsetNotProvidedNumerator"))]
    pub age_of_onset_not_provided_numerator: i32,
    #[serde(rename(deserialize = "AgeOfOnsetNotProvidedDenominator"))]
    pub age_of_onset_not_provided_denominator: i32,
    #[serde(rename(deserialize = "AgeOfOnsetOtherNumerator"))]
    pub age_of_onset_other_numerator: i32,
    #[serde(rename(deserialize = "AgeOfOnsetOtherDenominator"))]
    pub age_of_onset_other_denominator: i32,
    #[serde(rename(deserialize = "CodeType"))]
    pub code_type: Option<CodeType>,
    #[serde(rename(deserialize = "CodeValue"))]
    pub code_value: Option<String>,
    #[serde(rename(deserialize = "ConditionIDType"))]
    pub condition_id_type: Option<ConditionIDType>,
    #[serde(rename(deserialize = "ConditionIDValue"))]
    pub condition_id_value: Option<String>,
    #[serde(rename(deserialize = "PreferredConditionName"))]
    pub preferred_condition_name: Option<String>,
    #[serde(rename(deserialize = "DiseaseName"))]
    pub disease_name: Option<String>,
    #[serde(rename(deserialize = "DiseaseArea1"))]
    pub disease_area_1: Option<DiseaseArea1>,
    #[serde(rename(deserialize = "DiseaseArea2"))]
    pub disease_area_2: Option<DiseaseArea2>,
    #[serde(rename(deserialize = "DataOrigin"))]
    pub data_origin: Option<DataOrigin>,
    #[serde(rename(deserialize = "AlleleOrigin"))]
    pub allele_origin: Option<AlleleOrigin>,
    /// /MGS\d{6}/ or "not provided"
    #[serde(default="default_resource", rename(deserialize = "SubmissionID"))]
    pub submission_id: String,
    #[serde(rename(deserialize = "SubmitterName1"))]
    pub submitter_name_1: Option<String>,
    #[serde(rename(deserialize = "SubmitterInstitute1"))]
    pub submitter_institute_1: Option<String>,
    #[serde(rename(deserialize = "SubmitterName2"))]
    pub submitter_name_2: Option<String>,
    #[serde(rename(deserialize = "SubmitterInstitute2"))]
    pub submitter_institute_2: Option<String>,
    /// Comma-separated list
    #[serde(rename(deserialize = "Citation"))]
    pub citation: Option<String>,
}

fn default_resource() -> String {
    "".to_string()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ClinicalSignificance {
    Benign,
    Pathogenic,
    #[serde(rename = "Uncertain_significance")]
    UncertainSignificance,
    #[serde(rename = "not_provided")]
    NotProvided,
    #[serde(rename = "Likely_benign")]
    LikelyBenign,
    #[serde(rename = "Likely_pathogenic")]
    LikelyPathogenic,
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "drug_response")]
    DrugResponse,
}

#[derive(Debug, Deserialize)]
pub enum CodeType {
    ICD10,
    #[serde(rename(deserialize = "SNOMED CT"))]
    SnomedCt,
}

#[derive(Debug, Deserialize)]
pub enum ConditionIDType {
    MeSH,
    MedGen,
    OMIM,
    HPO,
    Orphanet,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DiseaseArea1 {
    Cancer,
    Dementia,
    #[serde(rename = "Infectious_diseases")]
    InfectiousDiseases,
    #[serde(rename = "Others")]
    Others,
    #[serde(rename = "Rare/Intractable_diseases")]
    RareIntractableDiseases,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DiseaseArea2 {
    Germline,
    HIV,
    #[serde(rename = "HTLV-1")]
    HTLV1,
    Hepatitis,
    Somatic,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum DataOrigin {
    #[serde(rename(deserialize = "gwas"))]
    GWAS,
    #[serde(rename(deserialize = "variant"))]
    Variant,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum AlleleOrigin {
    Germline,
    Somatic,
    Unknown,
    #[serde(rename(deserialize = "not provided"))]
    NotProvided,
}
