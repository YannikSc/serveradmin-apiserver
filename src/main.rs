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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let sa_converter = ServeradminConverter::new();
    let app = App {
        serveradmin_converter: sa_converter.clone(),
        kube_converter: KubeConverter {},
        data_api: ServeradminDataApi { sa_converter },
    };

    let router = axum::Router::new()
        .nest("/", controller::kube_apis::router())
        .nest("/openapi", controller::openapi::router())
        .with_state(app);

    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:8081").await?,
        router.into_make_service(),
    )
    .await?;

    Ok(())
}
