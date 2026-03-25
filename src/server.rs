use crate::model::AppState;
use crate::routes;
use axum::Router;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

pub async fn run(state: AppState) {
    let addr = state.config.server_addr;
    let state = Arc::new(state);

    let app = Router::new()
        .merge(routes::router())
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    let listener = TcpListener::bind(addr).await.expect("failed to bind port");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .expect("failed to serve");
}
