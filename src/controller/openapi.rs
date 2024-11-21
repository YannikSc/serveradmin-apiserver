use axum::{response::IntoResponse, routing::get, Json};

#[axum::debug_handler]
async fn openapi_v3() -> impl IntoResponse {
    Json(serde_json::json! {{
        "paths": {
            "apis/serveradmin.innogames.de/v1": {
                "serverRelativeURL": "/openapi/v3/apis/serveradmin.innogames.de/v1",
            },
        },
    }})
}

#[axum::debug_handler]
async fn openapi_v3_serveradmin_apis() -> impl IntoResponse {
    String::new()
}

/// Can respond empty but has to exist
#[axum::debug_handler]
async fn openapi_v2() -> impl IntoResponse {
    String::new()
}

pub fn router() -> axum::Router<crate::App> {
    axum::Router::new()
        .route("/v2", get(openapi_v2))
        .route("/v3", get(openapi_v3))
        .route(
            "/v3/apis/serveradmin.innogames.de/v1",
            get(openapi_v3_serveradmin_apis),
        )
}
