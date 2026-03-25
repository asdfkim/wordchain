use crate::config::Config;
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub tx: broadcast::Sender<String>,
}
