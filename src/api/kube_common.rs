use std::collections::HashMap;

pub type AnySpec = HashMap<String, serde_json::Value>;

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CommonMeta {
    #[serde(skip_serializing_if = "String::is_empty")]
    pub api_version: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub kind: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonMetadata {
    pub name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub namespace: String,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub labels: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub annotations: HashMap<String, serde_json::Value>,
    #[serde(flatten, default)]
    pub _other: serde_json::Value,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct AnyManifest {
    #[serde(flatten)]
    pub type_meta: CommonMeta,
    pub metadata: CommonMetadata,
    pub spec: AnySpec,
    #[serde(flatten)]
    pub _other: serde_json::Value,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct AnyList<I> {
    #[serde(flatten)]
    pub type_meta: CommonMeta,
    pub metadata: CommonMetadata,
    pub items: Vec<I>,
    #[serde(flatten)]
    pub _other: serde_json::Value,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TableColumnType {
    Integer,
    Number,
    #[default]
    String,
    Boolean,
    Date,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum TableColumnFormat {
    Name,
    Int32,
    Int64,
    Float,
    Double,
    Byte,
    Date,
    DateTime,
    Password,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableColumnDefinition {
    pub name: String,
    pub r#type: TableColumnType,
    pub format: Option<TableColumnFormat>,
    pub description: String,
    pub priority: i32,
    #[serde(skip)]
    pub attribute_id: String,
    pub cell_content: Option<CellContent>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum CellContent {
    Pointer(String),
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRow<T = serde_json::Value> {
    pub cells: Vec<serde_json::Value>,
    pub conditions: Vec<TableRowCondition>,
    pub object: T,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum ConditionStatus {
    True,
    False,
    #[default]
    Unknown,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TableRowCondition {
    pub r#type: String,
    pub status: ConditionStatus,
    pub reason: String,
    pub message: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaTable<T> {
    #[serde(flatten)]
    pub type_meta: CommonMeta,
    pub metadata: CommonMetadata,
    pub column_definitions: Vec<TableColumnDefinition>,
    pub rows: Vec<TableRow<T>>,
}

impl CellContent {
    pub fn extract_cell_value(&self, row: &serde_json::Value) -> serde_json::Value {
        match self {
            CellContent::Pointer(pointer) => row
                .pointer(pointer)
                .cloned()
                .unwrap_or_else(|| serde_json::Value::String("N/A".to_string())),
        }
    }
}

impl<T: serde::Serialize> MetaTable<T> {
    pub fn try_new(
        columns: Vec<TableColumnDefinition>,
        cells: Vec<CellContent>,
        items: Vec<T>,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            type_meta: CommonMeta {
                api_version: "meta.k8s.io/v1".to_string(),
                kind: "Table".to_string(),
            },
            metadata: Default::default(),
            column_definitions: columns,
            rows: items
                .into_iter()
                .map(|item| {
                    let object_data = serde_json::to_value(&item)?;

                    Ok(TableRow {
                        cells: cells
                            .iter()
                            .map(|cell| cell.extract_cell_value(&object_data))
                            .collect::<Vec<_>>(),
                        conditions: vec![],
                        object: item,
                    })
                })
                .collect::<anyhow::Result<Vec<_>>>()?,
        })
    }
}

impl CommonMeta {
    pub fn new_serveradmin(kind: impl ToString) -> Self {
        Self {
            api_version: "serveradmin.innogames.de/v1".to_string(),
            kind: kind.to_string(),
        }
    }
}
