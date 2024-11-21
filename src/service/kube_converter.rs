use adminapi::api::Server;
use axum::Json;
use convert_case::{Case, Casing};

use crate::api::kube_common::{AnyList, AnyManifest, CommonMeta, CommonMetadata, MetaTable};

#[derive(Clone)]
pub struct KubeConverter {}

impl KubeConverter {
    pub fn servertype_to_common_meta(&self, servertype: &str) -> CommonMeta {
        CommonMeta {
            api_version: "serveradmin.innogames.de/v1".to_string(),
            kind: servertype.to_case(Case::Pascal),
        }
    }

    pub fn servers_to_metatable(
        &self,
        servers: Vec<Server>,
    ) -> anyhow::Result<MetaTable<AnyManifest>> {
        let servers = servers
            .into_iter()
            .map(|server| self.server_to_manifest(server))
            .collect::<anyhow::Result<Vec<_>>>()?;

        MetaTable::try_new(vec![], vec![], servers)
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

        Ok(AnyManifest {
            type_meta: self.servertype_to_common_meta(&servertype),
            metadata: CommonMetadata {
                name: hostname,
                namespace: project.to_string(),
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
}
