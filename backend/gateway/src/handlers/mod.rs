pub(crate) mod auth_handler;
pub(crate) mod product_handler;
pub(crate) mod user_handler;

pub(super) use crate::state::AppState;
pub(super) use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
pub(super) use serde_json::json;
