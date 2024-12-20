use axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
    Json,
};

use crate::App;

#[derive(Clone, Debug)]
pub struct Authorization {
    pub token: String,
}

#[axum::async_trait]
impl FromRequestParts<App> for Authorization {
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, state: &App) -> Result<Self, Self::Rejection> {
        let Some(authorization) = parts.headers.get(AUTHORIZATION) else {
            return Err((
                StatusCode::UNAUTHORIZED,
                state
                    .kube_converter
                    .error_status_response(401, "Missing authorization"),
            ));
        };

        Ok(Self {
            token: authorization
                .to_str()
                .unwrap_or_default()
                .trim_start_matches("Bearer ")
                .to_string(),
        })
    }
}
