use axum::{
    routing::{get, post},
    Router,
};
use lambda_http::{run, Error};

mod model;
mod web;

use model::item::ModelManager;
use web::test_handlers::{get_foo, post_foo, post_foo_name, root};

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    let config = aws_config::load_from_env().await;
    let mm = ModelManager::new("rs-lambda-dynamo-db", &config);

    let app = Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo).post(post_foo))
        .route("/foo/:name", post(post_foo_name))
        .route(
            "/db",
            get(web::item_handlers::query)
                .post(web::item_handlers::insert)
                .with_state(mm),
        );
    // .route("/db/query", get(web::item_handlers::query));

    run(app).await
}
