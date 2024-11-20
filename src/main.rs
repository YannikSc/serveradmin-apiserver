mod controller;
mod service;

#[derive(Clone)]
pub struct App {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = App {};

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
