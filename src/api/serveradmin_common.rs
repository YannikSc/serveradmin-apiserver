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
