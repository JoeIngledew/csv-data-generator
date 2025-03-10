use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldIntRange {
    pub min: i64,
    pub max: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    String,
    Number,
    Boolean,
    Date,
    Uuid,
    PatternString(String),
    Name,
    Selection(Vec<String>),
    RangeInteger(FieldIntRange),
    CompanyBrand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputDefItem {
    pub field_name: String,
    pub field_type: FieldType,
    pub order: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputDefinition {
    pub fields: Vec<InputDefItem>,
}

pub struct CsvOutput {
    pub header: Vec<String>,
    pub rows: Vec<Vec<String>>,
}
