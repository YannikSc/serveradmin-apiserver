use axum::{response::IntoResponse, routing::get};

/// Can respond empty but has to exist
#[axum::debug_handler]
async fn openapi_v2() -> impl IntoResponse {
    String::new()
}

pub fn router() -> axum::Router<crate::App> {
    axum::Router::new().route("/v2", get(openapi_v2))
}
