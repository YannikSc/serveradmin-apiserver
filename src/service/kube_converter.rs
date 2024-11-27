use std::{collections::HashMap, sync::Arc};

use adminapi::api::Server;
use axum::Json;
use convert_case::{Case, Casing};

use crate::api::{
    kube_common::{
        AnyList, AnyManifest, CellContent, CommonMeta, CommonMetadata, MetaTable,
        TableColumnDefinition, TableColumnFormat, TableColumnType,
    },
    serveradmin_common::{PredefinedColumn, DEFAULT_ATTRIBUTES, VISIBLE_ATTRIBUTES},
    servertypes::{Attribute, AttributeType},
};

#[cfg(feature = "advanced_metadata_storage")]
use super::serveradmin_data_api::from_key_value;

#[derive(Clone)]
pub struct KubeConverter {
    pub attributes: Arc<Vec<Attribute>>,
    pub servertypes: Arc<HashMap<String, Vec<Attribute>>>,
}

impl KubeConverter {
    pub fn get_api_resources(&self) -> serde_json::Value {
        use convert_case::{Case, Casing};

        let resources = self
            .servertypes
            .iter()
            .map(|(servertype, attributes)| {
                let name = servertype.to_case(Case::Flat) + "s";
                let singular_name = servertype.to_case(Case::Flat);
                let kind = servertype.to_case(Case::Pascal);
                let namespaced = attributes
                    .iter()
                    .find(|attr| attr.name.eq("project"))
                    .is_some();
                let mut title = servertype.split("_");
                let mut short_name = String::new();
                while let Some(word) = title.next() {
                    if let Some(char) = word.chars().next() {
                        short_name.push(char);
                    }
                }

                let mut short_names = Vec::new();

                if short_name.len() > 1 {
                    short_names.push(short_name);
                }

                serde_json::json! {{
                    "name": name,
                    "singularName": singular_name,
                    "namespaced": namespaced,
                    "kind": kind,
                    "verbs": [
                        "get"
                    ],
                    "shortNames": short_names,
                }}
            })
            .collect::<Vec<_>>();

        serde_json::json! {{
            "kind": "APIResourceList",
            "apiVersion": "v1",
            "groupVersion": "serveradmin.innogames.de/v1",
            "resources": resources,
        }}
    }

    pub fn servertype_to_common_meta(&self, servertype: &str) -> CommonMeta {
        CommonMeta {
            api_version: "serveradmin.innogames.de/v1".to_string(),
            kind: servertype.to_case(Case::Pascal),
        }
    }

    pub fn servers_to_metatable(
        &self,
        servers: Vec<Server>,
        kind: &str,
    ) -> anyhow::Result<MetaTable<AnyManifest>> {
        let servers = servers
            .into_iter()
            .map(|server| self.server_to_manifest(server))
            .collect::<anyhow::Result<Vec<_>>>()?;

        let columns = self.get_column_definitions(kind)?;
        let values = columns
            .iter()
            .map(|column| {
                column
                    .cell_content
                    .clone()
                    .unwrap_or(CellContent::Pointer(format!(
                        "/spec/{}",
                        column.attribute_id
                    )))
            })
            .collect();
        MetaTable::try_new(columns, values, servers)
    }

    pub fn servers_to_list(
        &self,
        servertype: &str,
        servers: Vec<Server>,
    ) -> anyhow::Result<AnyList<AnyManifest>> {
        let meta = self.servertype_to_common_meta(servertype);
        let servers = servers
            .into_iter()
            .map(|server| self.server_to_manifest(server))
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(AnyList {
            type_meta: CommonMeta {
                api_version: "serveradmin.innogames.de/v1".to_string(),
                kind: meta.kind + "List",
            },
            metadata: Default::default(),
            items: servers,
            ..Default::default()
        })
    }

    pub fn server_to_manifest(&self, server: Server) -> anyhow::Result<AnyManifest> {
        let serde_json::Value::String(servertype) = server.get("servertype") else {
            return Err(anyhow::anyhow!("Required property servertype unknown"));
        };
        let serde_json::Value::String(hostname) = server.get("hostname") else {
            return Err(anyhow::anyhow!("Required property hostname unknown"));
        };
        let project = server.get("project");
        let project = project.as_str().unwrap_or_default();

        #[cfg(feature = "advanced_metadata_storage")]
        let labels = serde_json::from_value::<Vec<String>>(server.get("labels"))
            .map(from_key_value)
            .unwrap_or_default();
        #[cfg(feature = "advanced_metadata_storage")]
        let annotations = serde_json::from_value::<Vec<String>>(server.get("annotations"))
            .map(from_key_value)
            .unwrap_or_default();

        Ok(AnyManifest {
            type_meta: self.servertype_to_common_meta(&servertype),
            metadata: CommonMetadata {
                name: hostname,
                namespace: project.to_string(),
                #[cfg(feature = "advanced_metadata_storage")]
                labels,
                #[cfg(feature = "advanced_metadata_storage")]
                annotations,
                ..Default::default()
            },
            spec: serde_json::from_value(serde_json::to_value(&server.attributes)?)?,
            ..Default::default()
        })
    }

    pub fn error_status_response(
        &self,
        code: i32,
        error: impl ToString,
    ) -> Json<serde_json::Value> {
        Json(serde_json::json! {{
            "kind": "Status",
            "code": code,
            "apiVersion": "v1",
            "message": error.to_string(),
            "details": {
                "causes": [{"field": "patch", "reason": "GenericError", "message": error.to_string()}],
            },
            "reason": "Invalid",
            "status": "Failure",
        }})
    }

    fn get_column_definitions(&self, kind: &str) -> anyhow::Result<Vec<TableColumnDefinition>> {
        let visible_columns = VISIBLE_ATTRIBUTES
            .iter()
            .find(|(name, _)| name.eq(&kind))
            .map(|(_, cols)| *cols)
            .unwrap_or(DEFAULT_ATTRIBUTES);
        let mut columns = Vec::new();

        for column in visible_columns.iter() {
            let Some(attribute) = self
                .attributes
                .iter()
                .find(|attr| attr.name.eq(&column.get_name()))
            else {
                tracing::error!(
                    "Unable to find attribute for predefined column {}",
                    column.get_name()
                );

                continue;
            };

            let priority = match column {
                PredefinedColumn::Always(_) => 0,
                PredefinedColumn::Detailed(_) => 1,
            };

            let name = column.get_name();
            columns.push(
                self.get_column_definition_for_column(attribute, priority)
                    .ok_or(anyhow::anyhow!(
                        "Unable to resolve table column from attribute {name}"
                    ))?,
            );
        }

        return Ok(columns);
    }

    fn get_column_definition_for_column(
        &self,
        attribute: &Attribute,
        priority: i32,
    ) -> Option<TableColumnDefinition> {
        use convert_case::{Case, Casing};

        let typ = match attribute.typ {
            AttributeType::Boolean => TableColumnType::Boolean,
            AttributeType::Date | AttributeType::Datetime => TableColumnType::Date,
            AttributeType::Number => TableColumnType::Number,
            _ => TableColumnType::String,
        };

        let format = match attribute.typ {
            AttributeType::Number => Some(TableColumnFormat::Int64),
            AttributeType::Date => Some(TableColumnFormat::Date),
            AttributeType::Datetime => Some(TableColumnFormat::DateTime),
            _ => None,
        };

        Some(TableColumnDefinition {
            name: attribute.name.to_case(Case::Pascal),
            r#type: typ,
            description: attribute.hovertext.clone(),
            format,
            priority,
            attribute_id: attribute.name.clone(),
            ..Default::default()
        })
    }
}
