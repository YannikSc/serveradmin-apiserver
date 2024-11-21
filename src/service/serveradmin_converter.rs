use convert_case::{Case, Casing};

use crate::api::kube_common::CommonMeta;

pub struct ServeradminConverter {}

impl ServeradminConverter {
    pub fn kind_to_servertype(&self, kind: &str) -> String {
        kind.to_case(Case::Snake)
    }

    pub fn servertype_to_common_meta(&self, servertype: &str) -> CommonMeta {
        CommonMeta {
            api_version: "serveradmin.innogames.de/v1".to_string(),
            kind: servertype.to_case(Case::Pascal),
        }
    }
}
