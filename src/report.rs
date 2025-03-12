use serde::{Deserialize, Serialize};

// use crate::components::{aggregate::Aggregate, granularity::Granularity};


#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PointRef {
    pub namespace: String,
    #[serde(rename = "ref")]
    pub ref_info: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Report {
    pub name: String,
    pub enabled: bool,
    pub report_style: String,
    pub site_group: String, // Used for permissions 
    pub point_refs: Vec<PointRef>,
    pub days_back: usize,
    //pub granularity: Granularity,
    //pub aggregation: Aggregate,
    pub recipients: Vec<String>,
    pub schedule: String,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoredReport {
    pub report: Report,
    pub generated_reports: Vec<String>,
}