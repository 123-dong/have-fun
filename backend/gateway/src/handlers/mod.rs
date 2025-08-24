pub(crate) mod auth;
pub(crate) mod product;
pub(crate) mod user;

pub(super) use crate::state::AppState;
pub(super) use axum::{
    Json,
    extract::{Path, State},
};
pub(super) use serde_json::json;
