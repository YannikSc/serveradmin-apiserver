pub mod kube_common;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct RequestContext {
    /// Contains the Bearer token, that is sent to the server in the Authorization header
    /// It is supposed to be a valid Serveradmin token.
    pub token: String,
}
