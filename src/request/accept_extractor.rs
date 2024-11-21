use axum::{
    extract::FromRequestParts,
    http::{header::ACCEPT, request::Parts, StatusCode},
    Json,
};

use crate::App;

pub struct Accept {
    pub content_type: String,
}

#[axum::async_trait]
impl FromRequestParts<App> for Accept {
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, state: &App) -> Result<Self, Self::Rejection> {
        let Some(content_type) = parts.headers.get(ACCEPT) else {
            return Err((
                StatusCode::BAD_REQUEST,
                state
                    .kube_converter
                    .error_status_response(400, "Missing content_type"),
            ));
        };

        Ok(Self {
            content_type: content_type.to_str().unwrap_or_default().to_string(),
        })
    }
}

impl Accept {
    pub fn is_list(&self) -> bool {
        self.content_type == "application/json"
    }

    pub fn is_table(&self) -> bool {
        self.content_type
            .contains("application/json;as=Table;v=v1;g=meta.k8s.io")
    }
}
