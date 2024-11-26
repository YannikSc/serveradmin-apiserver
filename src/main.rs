use std::sync::Arc;

use service::{
    kube_converter::KubeConverter, serveradmin_converter::ServeradminConverter,
    serveradmin_data_api::ServeradminDataApi,
};

mod api;
mod controller;
mod request;
mod service;

#[derive(Clone)]
pub struct App {
    pub serveradmin_converter: ServeradminConverter,
    pub kube_converter: KubeConverter,
    pub data_api: ServeradminDataApi,
    pub openapi_definition: utoipa::openapi::OpenApi,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let servertypes = Arc::new(crate::api::servertypes::servertypes());
    let sa_converter = ServeradminConverter::new(servertypes.clone());
    let app = App {
        serveradmin_converter: sa_converter.clone(),
        kube_converter: KubeConverter {
            attributes: Arc::new(api::servertypes::attributes()),
            servertypes,
        },
        data_api: ServeradminDataApi { sa_converter },
        openapi_definition: crate::api::servertypes::openapi(),
    };

    let router = axum::Router::new()
        .nest("/", controller::kube_apis::router())
        .nest("/openapi", controller::openapi::router())
        .with_state(app);

    let listen_addr = std::env::var("LISTEN_ADDR").unwrap_or_else(|_| String::from("127.0.0.1:8080"));
    axum::serve(
        tokio::net::TcpListener::bind(listen_addr).await?,
        router.into_make_service(),
    )
    .await?;

    Ok(())
}
