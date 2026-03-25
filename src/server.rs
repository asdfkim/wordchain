use crate::model::AppState;
use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub async fn run(state: AppState) {
    let addr = state.config.server_addr;

    let app = Router::new().with_state(state);

    let listener = TcpListener::bind(addr).await.expect("failed to bind port");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("failed to serve");
}
