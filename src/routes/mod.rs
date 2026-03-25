mod ws;

use crate::model::AppState;
use axum::Router;
use std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new().nest("/ws", ws::router())
}
