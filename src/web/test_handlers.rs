use axum::{extract::Path, response::Json};
use serde_json::{json, Value};

pub async fn root() -> Json<Value> {
    Json(json!({ "msg": "I am GET /" }))
}

pub async fn get_foo() -> Json<Value> {
    Json(json!({ "msg": "I am GET /foo" }))
}

pub async fn post_foo(Json(some_val): Json<Value>) -> Json<Value> {
    Json(json!({ "msg": "I am POST /foo", "some_val": some_val }))
}

pub async fn post_foo_name(Path(name): Path<String>) -> Json<Value> {
    Json(json!({ "msg": format!("I am POST /foo/:name, name={name}") }))
}
