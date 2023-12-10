use axum::{extract::State, response::Json};
use serde_json::{json, Value};

use crate::model::item::{Item, ModelManager};

pub async fn get_all(State(mm): State<ModelManager>) -> Json<Vec<Item>> {
    Json(mm.scan().await.unwrap())
}

pub async fn insert(State(mm): State<ModelManager>, Json(item): Json<Item>) -> Json<Value> {
    mm.put(item).await.unwrap();
    Json(json!({"msg":"Item successfully created."}))
}
