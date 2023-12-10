use aws_sdk_dynamodb::Client;
use axum::{
    extract::{Path, State},
    response::Json,
    routing::{get, post},
    Router,
};
use lambda_http::{run, Error};
use serde::{Deserialize, Serialize};
use serde_dynamo::{from_items, to_attribute_value};
use serde_json::{json, Value};
use tracing::info;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub account_type: String,
    pub age: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

async fn get_all(State(db_client): State<Client>) -> Json<Vec<Item>> {
    let result = db_client
        .scan()
        .table_name("rs-lambda-dynamo-db")
        .send()
        .await
        .unwrap();

    let items_result = result.items().to_vec();
    let items: Vec<Item> = from_items(items_result).unwrap();

    Json(items)
}

async fn insert(State(db_client): State<Client>, Json(item): Json<Item>) -> Json<Item> {
    add_item(&db_client, item.clone(), "rs-lambda-dynamo-db")
        .await
        .unwrap();

    Json(item)
}

async fn root() -> Json<Value> {
    Json(json!({ "msg": "I am GET /" }))
}

async fn get_foo() -> Json<Value> {
    Json(json!({ "msg": "I am GET /foo" }))
}

async fn post_foo(Json(some_val): Json<Value>) -> Json<Value> {
    Json(json!({ "msg": "I am POST /foo", "some_val": some_val }))
}

async fn post_foo_name(Path(name): Path<String>) -> Json<Value> {
    Json(json!({ "msg": format!("I am POST /foo/:name, name={name}") }))
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let config = aws_config::load_from_env().await;
    let db_client = Client::new(&config);

    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/:name", post(post_foo_name))
        .route("/db", get(get_all).post(insert).with_state(db_client));

    run(app).await
}

pub async fn add_item(client: &Client, item: Item, table: &str) -> Result<(), Error> {
    let user_av = to_attribute_value(item.username)?;
    let type_av = to_attribute_value(item.account_type)?;
    let age_av = to_attribute_value(item.age)?;
    let first_av = to_attribute_value(item.first_name)?;
    let last_av = to_attribute_value(item.last_name)?;

    let request = client
        .put_item()
        .table_name(table)
        .item("username", user_av)
        .item("account_type", type_av)
        .item("age", age_av)
        .item("first_name", first_av)
        .item("last_name", last_av);

    info!("adding item to DynamoDB");

    let _resp = request.send().await?;

    Ok(())
}
