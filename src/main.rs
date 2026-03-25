use std::sync::Arc;
use tokio::sync::broadcast;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use wordchain::config::Config;
use wordchain::model::AppState;
use wordchain::server;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Arc::new(Config::from_env());
    let (tx, _rx) = broadcast::channel(16);

    let state = AppState { config, tx };

    server::run(state).await;
}
