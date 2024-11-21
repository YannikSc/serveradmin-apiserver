use std::collections::HashMap;

pub type AnySpec = HashMap<String, serde_json::Value>;

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonMeta {
    pub api_version: String,
    pub kind: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommonMetadata {
    pub name: String,
    pub namespace: String,
    pub labels: HashMap<String, serde_json::Value>,
    pub annotations: HashMap<String, serde_json::Value>,
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

impl<T: serde::Serialize> MetaTable<T> {
    pub fn try_new(
        columns: Vec<TableColumnDefinition>,
        cells: Vec<String>,
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
                            .map(|cell| {
                                object_data
                                    .pointer(cell)
                                    .cloned()
                                    .unwrap_or_else(|| serde_json::Value::String(cell.to_string()))
                            })
                            .collect::<Vec<_>>(),
                        conditions: vec![],
                        object: item,
                    })
                })
                .collect::<anyhow::Result<Vec<_>>>()?,
        })
    }
}
