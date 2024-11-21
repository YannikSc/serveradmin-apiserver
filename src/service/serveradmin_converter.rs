use std::{collections::HashMap, sync::Arc};

use convert_case::{Case, Casing};

use crate::api::serveradmin_common::Attribute;

#[derive(Clone)]
pub struct ServeradminConverter {
    servertypes: Arc<HashMap<String, Vec<Attribute>>>,
}

impl ServeradminConverter {
    pub fn new() -> Self {
        Self {
            servertypes: Default::default(),
        }
    }

    pub fn plural_to_servertype(&self, plural: &str) -> Option<String> {
        let (servertype, _) = self
            .servertypes
            .iter()
            .find(|(servertype, _)| servertype.to_case(Case::Flat) + "s" == plural)?;

        Some(servertype.to_string())
    }

    pub fn kind_to_servertype(&self, kind: &str) -> String {
        kind.to_case(Case::Snake)
    }
}
