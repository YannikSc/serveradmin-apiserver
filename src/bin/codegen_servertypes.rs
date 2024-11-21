use std::collections::HashMap;

use convert_case::{Case, Casing};
use raw_csv::RawAttribute;

pub const SERVERTYPES: &str = include_str!("../../resources/servertypes.csv");
pub const ATTRIBUTES: &str = include_str!("../../resources/attributes.csv");
pub const SERVERTYPE_ATTRIBUTES: &str = include_str!("../../resources/servertype_attributes.csv");

mod raw_csv {
    use super::AttributeType;

    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct RawAttribute {
        pub attribute_id: String,
        pub r#type: AttributeType,
        pub reversed_attribute_id: String,
        pub multi: bool,
        pub regexp: String,
        pub readonly: bool,
        pub hovertext: String,
    }

    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct RawServertype {
        pub servertype_id: String,
        pub ip_addr_type: String,
    }

    #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
    pub struct RawServertypeAttribute {
        pub servertype_id: String,
        pub attribute_id: String,
        pub required: bool,
        pub default_value: String,
    }
}

#[derive(Clone)]
pub struct ServertypeBuilder {
    pub servertypes: HashMap<String, Servertype>,
}

#[derive(Clone, Debug)]
pub struct Servertype {
    pub name: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Clone, Debug, serde::Deserialize, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributeType {
    Domain,   // -> String
    Date,     // -> String
    Number,   // -> Number
    Inet,     // -> String
    Supernet, // -> String
    Macaddr,  // -> String
    Boolean,  // -> Boolean
    Relation, // -> String
    String,   // -> String
    Datetime, // -> Date
    Reverse,  // -> ?
}

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub r#type: AttributeType,
    pub reversed_attribute_id: String,
    pub multi: bool,
    pub regexp: String,
    pub readonly: bool,
    pub hovertext: String,
    pub required: bool,
    pub default: String,
}

impl ServertypeBuilder {
    pub fn try_new(
        servertypes: String,
        attributes: String,
        servertype_attributes: String,
    ) -> anyhow::Result<Self> {
        let servertypes = Self::parse_servertypes(servertypes)?;
        let mut attributes = Self::parse_attributes(attributes)?;
        let mut servertype_attributes = Self::parse_servertype_attributes(servertype_attributes)?;

        attributes.push(RawAttribute {
            attribute_id: "ipv4".to_string(),
            r#type: AttributeType::Inet,
            reversed_attribute_id: String::new(),
            multi: false,
            regexp: String::new(),
            readonly: false,
            hovertext: "The almighty IPv4 address. Comes from somewhere, good luck finding it!"
                .to_string(),
        });
        attributes.push(RawAttribute {
            attribute_id: "ipv6".to_string(),
            r#type: AttributeType::Inet,
            reversed_attribute_id: String::new(),
            multi: false,
            regexp: String::new(),
            readonly: false,
            hovertext: "The almighty IPv6 address. Comes from somewhere, good luck finding it!"
                .to_string(),
        });
        attributes.push(RawAttribute {
            attribute_id: "hostname".to_string(),
            r#type: AttributeType::Inet,
            reversed_attribute_id: String::new(),
            multi: false,
            regexp: String::new(),
            readonly: false,
            hovertext:
                "The almighty hostname attribute. Comes from somewhere, good luck finding it!"
                    .to_string(),
        });
        attributes.push(RawAttribute {
            attribute_id: "servertype".to_string(),
            r#type: AttributeType::Inet,
            reversed_attribute_id: String::new(),
            multi: false,
            regexp: String::new(),
            readonly: false,
            hovertext:
                "The almighty servertype attribute. Comes from somewhere, good luck finding it!"
                    .to_string(),
        });
        attributes.push(RawAttribute {
            attribute_id: "object_id".to_string(),
            r#type: AttributeType::Inet,
            reversed_attribute_id: String::new(),
            multi: false,
            regexp: String::new(),
            readonly: false,
            hovertext:
                "The almighty object_id attribute. Comes from somewhere, good luck finding it!"
                    .to_string(),
        });

        for servertype in &servertypes {
            if servertype.ip_addr_type != "null" {
                servertype_attributes.push(raw_csv::RawServertypeAttribute {
                    servertype_id: servertype.servertype_id.clone(),
                    attribute_id: "ipv4".to_string(),
                    required: true,
                    default_value: String::new(),
                });
                servertype_attributes.push(raw_csv::RawServertypeAttribute {
                    servertype_id: servertype.servertype_id.clone(),
                    attribute_id: "ipv6".to_string(),
                    required: true,
                    default_value: String::new(),
                });
                servertype_attributes.push(raw_csv::RawServertypeAttribute {
                    servertype_id: servertype.servertype_id.clone(),
                    attribute_id: "intern_ip".to_string(),
                    required: true,
                    default_value: String::new(),
                });
            }

            servertype_attributes.push(raw_csv::RawServertypeAttribute {
                servertype_id: servertype.servertype_id.clone(),
                attribute_id: "hostname".to_string(),
                required: true,
                default_value: String::new(),
            });

            servertype_attributes.push(raw_csv::RawServertypeAttribute {
                servertype_id: servertype.servertype_id.clone(),
                attribute_id: "servertype".to_string(),
                required: true,
                default_value: String::new(),
            });

            servertype_attributes.push(raw_csv::RawServertypeAttribute {
                servertype_id: servertype.servertype_id.clone(),
                attribute_id: "object_id".to_string(),
                required: true,
                default_value: String::new(),
            });
        }

        let attributes = attributes
            .into_iter()
            .map(|attr| (attr.attribute_id.clone(), attr))
            .collect::<HashMap<_, _>>();

        let mut structured_servertypes = HashMap::new();

        for servertype in servertypes {
            let my_attributes = servertype_attributes
                .iter()
                .filter(|attr| attr.servertype_id.eq(&servertype.servertype_id))
                .filter_map(|attr| {
                    let Some(raw_attr) = attributes.get(&attr.attribute_id) else {
                        return None;
                    };

                    let mut typ = raw_attr.r#type.clone();

                    if raw_attr.r#type == AttributeType::Reverse {
                        let Some(attr) = attributes.get(&raw_attr.reversed_attribute_id) else {
                            log::error!(
                                "Unable to find item {} for reverse in {}",
                                raw_attr.reversed_attribute_id,
                                raw_attr.attribute_id
                            );

                            return None;
                        };

                        typ = attr.r#type.clone();
                    }

                    Some(Attribute {
                        name: raw_attr.attribute_id.clone(),
                        r#type: typ,
                        reversed_attribute_id: raw_attr.reversed_attribute_id.clone(),
                        multi: raw_attr.multi,
                        regexp: raw_attr.regexp.clone(),
                        readonly: raw_attr.readonly,
                        hovertext: raw_attr.hovertext.clone(),
                        required: attr.required,
                        default: attr.default_value.clone(),
                    })
                })
                .collect::<Vec<_>>();

            structured_servertypes.insert(
                servertype.servertype_id.clone(),
                Servertype {
                    name: servertype.servertype_id,
                    attributes: my_attributes,
                },
            );
        }

        Ok(Self {
            servertypes: structured_servertypes,
        })
    }

    pub fn get_api_group(&self) -> serde_json::Value {
        serde_json::json! {{
            "kind": "APIGroup",
            "apiVersion": "v1",
            "name": "serveradmin.innogames.de",
            "versions": [
                { "groupVersion": "serveradmin.innogames.de/v1", "version": "v1" },
            ],
            "preferredVersion": { "groupVersion": "serveradmin.innogames.de/v1", "version": "v1" },
        }}
    }

    pub fn api_resource_to_servertype(&self, resource_name: &str) -> anyhow::Result<String> {
        use convert_case::{Case, Casing};

        self.servertypes
            .iter()
            .find(|(_name, servertype)| {
                let plural = servertype.name.to_case(Case::Flat) + "s";

                plural.eq(resource_name)
            })
            .map(|(name, _servertype)| name.to_string())
            .ok_or(anyhow::anyhow!(
                "Unable to find servertype for {resource_name}"
            ))
    }

    pub fn get_attributes_for_servertype(&self, servertype: &str) -> anyhow::Result<Vec<String>> {
        self.servertypes
            .get(servertype)
            .map(|servertype| {
                servertype
                    .attributes
                    .iter()
                    .map(|attr| attr.name.clone())
                    .collect::<Vec<_>>()
            })
            .ok_or(anyhow::anyhow!("Unable to find servertype"))
    }

    fn parse_servertypes(raw: String) -> anyhow::Result<Vec<raw_csv::RawServertype>> {
        let mut reader = csv::Reader::from_reader(raw.as_bytes());

        Ok(reader.deserialize().collect::<csv::Result<_>>()?)
    }

    fn parse_attributes(raw: String) -> anyhow::Result<Vec<raw_csv::RawAttribute>> {
        let mut reader = csv::Reader::from_reader(raw.as_bytes());

        Ok(reader.deserialize().collect::<csv::Result<_>>()?)
    }

    fn parse_servertype_attributes(
        raw: String,
    ) -> anyhow::Result<Vec<raw_csv::RawServertypeAttribute>> {
        let mut reader = csv::Reader::from_reader(raw.as_bytes());

        Ok(reader.deserialize().collect::<csv::Result<_>>()?)
    }
}

fn create_servertype_spec_struct(servertype: &Servertype) -> String {
    let struc_name = servertype.name.to_case(Case::Pascal) + "Spec";
    let mut struc = format!("#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]\n#[schema(as = serveradmin::innogames::de::v1::{struc_name})]\n#[serde_inline_default::serde_inline_default]\n#[serde(rename_all = \"snake_case\")]\npub struct {struc_name} {{\n");

    for attr in &servertype.attributes {
        let rust_type = match attr.r#type {
            AttributeType::Number => "i32",
            AttributeType::Boolean => "bool",
            _ => "String",
        }
        .to_string();
        let mut typ = rust_type.clone();

        if attr.multi {
            typ = format!("Vec<{typ}>");
        }

        if !attr.required {
            typ = format!("Option<{typ}>");
        }

        let mut serde_attr = String::new();

        if !attr.default.is_empty() {
            let mut value = attr.default.clone();

            if rust_type == "String" {
                value = format!("\"{value}\"");
            }

            if rust_type == "bool" {
                value = value.eq("1").to_string();
            }

            value = format!("{rust_type}::from({value})");

            if attr.multi {
                value = format!("vec![{value}]");
            }

            if !attr.required {
                value = format!("Some({value})");
            }

            serde_attr = format!("#[serde_inline_default({value})]");
        }

        struc.push_str(&format!("    {serde_attr}{}: {typ},\n", attr.name));
    }

    struc.push_str("\n}");
    struc
}
fn create_servertype_list_struct(servertype: &Servertype) -> String {
    let struct_name = servertype.name.to_case(Case::Pascal) + "List";
    let item_type = servertype.name.to_case(Case::Pascal);
    let mut struc = format!("#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::{struct_name})] #[serde(rename_all = \"snake_case\")] pub struct {struct_name} {{\n");

    struc.push_str("    kind: String,\n");
    struc.push_str("    api_version: String,\n");
    struc.push_str(&format!("    items: {item_type},\n"));

    struc.push_str("\n}");
    struc
}

fn create_servertype_struct(servertype: &Servertype) -> String {
    let struct_name = servertype.name.to_case(Case::Pascal);
    let spec_type = servertype.name.to_case(Case::Pascal) + "Spec";
    let mut struc = format!("#[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)] #[schema(as = serveradmin::innogames::de::v1::{struct_name})] #[serde(rename_all = \"snake_case\")] pub struct {struct_name} {{\n");

    struc.push_str("    kind: String,\n");
    struc.push_str("    api_version: String,\n");
    struc.push_str("    metadata: ServerObjectMetadata,\n");
    struc.push_str(&format!("    spec: {spec_type},\n"));

    struc.push_str("\n}");
    struc
}

fn create_openapi_paths(servertype: &Servertype) -> String {
    let pluralize = servertype.name.to_case(Case::Flat) + "s";
    let kind = servertype.name.to_case(Case::Pascal);
    let list_kind = servertype.name.to_case(Case::Pascal) + "List";
    let api_base = "serveradmin.innogames.de.v1.";
    let hash = "#";
    let is_namespaced = servertype
        .attributes
        .iter()
        .find(|attribute| attribute.name.eq("project"))
        .is_some();
    let mut namespace = "";
    let mut namespace_parameter = "";

    if is_namespaced {
        namespace = "{namespace}";
        namespace_parameter = r#"Parameter::builder().name("namespace").schema(Some("string")).parameter_in(ParameterIn::Path).build(),"#;
    }

    format!(
        r#"
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/{pluralize}",
    PathItem::builder()
        .parameters(Some(vec![
            {namespace_parameter}
        ]))
        .operation(utoipa::openapi::HttpMethod::Get, Operation::builder()
            .response("200", Response::builder()
                .content("application/json", Content::builder().schema(Some(Ref::new("{hash}/components/schemas/{api_base}{list_kind}"))).build())
                .build(),
            )
        .build())
        .operation(utoipa::openapi::HttpMethod::Post, Operation::builder()
            .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("{hash}/components/schemas/{api_base}{kind}"))).build()).build()))
            .response(
                "200",
                Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("{hash}/components/schemas/{api_base}{list_kind}"))).build()).build(),
            )
            .build(),
        )
        .build(),
)
.path(
    "/apis/serveradmin.innogames.de/v1/namespace/{namespace}/{pluralize}/{{name}}",
    PathItem::builder()
        .parameters(Some(vec![
            {namespace_parameter}
            Parameter::builder().name("name").parameter_in(ParameterIn::Path).schema(Some("string")).build(),
        ]))
        .operation(
            utoipa::openapi::HttpMethod::Get,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("{hash}/components/schemas/{api_base}{kind}"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Delete,
            Operation::builder()
                .response(
                    "200",
                    Response::builder().content( "application/json", Content::builder().schema(Some(Ref::new( "{hash}/components/schemas/{api_base}{kind}"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Patch,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json", Content::builder().schema(Some(Ref::new("{hash}/components/schemas/{api_base}{kind}"))).build()).build(),))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("{hash}/components/schemas/{api_base}{kind}"))).build()).build(),
                )
                .build(),
        )
        .operation(
            utoipa::openapi::HttpMethod::Put,
            Operation::builder()
                .request_body(Some(RequestBody::builder().content("application/json",Content::builder().schema(Some(Ref::new("{hash}/components/schemas/{api_base}{kind}"))).build()).build()))
                .response(
                    "200",
                    Response::builder().content("application/json",Content::builder().schema(Some(Ref::new("{hash}/components/schemas/{api_base}{kind}"))).build()).build(),
                )
                .build(),
        )
        .build(),
)
    "#
    )
}

fn create_openapi_contents(servertype: &str) -> String {
    let kind = servertype.to_case(Case::Pascal);
    let list_kind = servertype.to_case(Case::Pascal) + "List";
    let spec_kind = servertype.to_case(Case::Pascal) + "Spec";

    format!(
        r#".schema_from::<{kind}>()
.schema_from::<{list_kind}>()
.schema_from::<{spec_kind}>()"#
    )
}

fn create_servertype_attributes(servertype: &Servertype) -> String {
    let attributes = servertype
        .attributes
        .iter()
        .map(|attribute| {
            format!(
                r#"vec.push(Attribute {{ name: "{}".to_string() }});"#,
                attribute.name
            )
        })
        .collect::<String>();
    format!(
        r#"servertypes.insert("{}".to_string(), {{ let mut vec = Vec::new(); {attributes} vec }});"#,
        servertype.name
    )
}

pub fn main() -> anyhow::Result<()> {
    let converter = ServertypeBuilder::try_new(
        SERVERTYPES.to_string(),
        ATTRIBUTES.to_string(),
        SERVERTYPE_ATTRIBUTES.to_string(),
    )?;

    let mut structs = Vec::new();
    let mut paths = Vec::new();
    let mut contents = Vec::new();
    let mut servertype_attributes = Vec::new();

    for (name, servertype) in &converter.servertypes {
        structs.push(create_servertype_spec_struct(servertype));
        structs.push(create_servertype_list_struct(servertype));
        structs.push(create_servertype_struct(servertype));
        paths.push(create_openapi_paths(servertype));
        contents.push(create_openapi_contents(name));
        servertype_attributes.push(create_servertype_attributes(servertype));
    }

    structs.push(
        stringify! {
            #[derive(Clone, Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
            #[serde(rename_all = "snake_case")]
            #[schema(as = serveradmin::innogames::de::v1::ServerObjectMetadata)]
            pub struct ServerObjectMetadata {
                name: String,
            }

            #[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
            pub struct Attribute {
                pub name: String,
            }
        }
        .to_string(),
    );

    let paths = paths.join("\n");
    let contents = contents.join("\n");
    structs.push(format!(r#"
use utoipa::openapi::{{path::{{Operation, Parameter, ParameterIn}}, request_body::RequestBody, Components, Content, PathItem, Paths, Ref, Response}};

pub fn openapi() -> utoipa::openapi::OpenApi {{
utoipa::openapi::OpenApi::builder()
    .paths(
        Paths::builder()
        {paths}
    )
    .components(Some(
        Components::builder()
            {contents}
            .schema_from::<ServerObjectMetadata>()
            .security_scheme(
                "bearerAuth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::builder()
                        .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                        .build(),
                ),
            )
            .build(),
    ))
    .security(Some(vec![utoipa::openapi::SecurityRequirement::new::<
        _,
        _,
        String,
    >("bearerAuth", vec![])]))
    .build()
}}
    "#));

    structs.push(format!(
        r#"pub fn servertypes() -> std::collections::HashMap<String, Vec<Attribute>> {{
        let mut servertypes = std::collections::HashMap::new();

        {}

        servertypes
}}"#,
        servertype_attributes.join("\n")
    ));

    let structs = structs.join("\n");

    std::fs::remove_file("src/api/servertypes.rs").ok();
    std::fs::write(
        "src/api/servertypes.rs",
        String::from(
            r#"///
/// This file is generated using the codegen_servertypes command!
///


"#,
        ) + &structs,
    )?;

    Ok(())
}
