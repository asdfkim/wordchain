use std::sync::Arc;
use wordchain::config::Config;
use wordchain::model::AppState;
use wordchain::server;

#[tokio::main]
async fn main() {
    let config = Arc::new(Config::from_env());
    let state = AppState { config };

    server::run(state).await;
}
