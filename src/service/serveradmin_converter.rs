use std::{collections::HashMap, sync::Arc};

use convert_case::{Case, Casing};

use crate::api::servertypes::Attribute;

#[derive(Clone)]
pub struct ServeradminConverter {
    servertypes: Arc<HashMap<String, Vec<Attribute>>>,
}

impl ServeradminConverter {
    pub fn new(servertypes: Arc<HashMap<String, Vec<Attribute>>>) -> Self {
        Self { servertypes }
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

    pub fn get_attribute_names_for_servertype(&self, servertype: &str) -> Vec<String> {
        self.servertypes
            .get(servertype)
            .map(|attrs| attrs.iter().map(|attr| attr.name.clone()).collect())
            .unwrap_or_default()
    }
}
