use aws_config::SdkConfig;
use aws_sdk_dynamodb::Client;
use lambda_http::Error;
use serde::{Deserialize, Serialize};
use serde_dynamo::from_items;
use serde_dynamo::to_attribute_value;
use tracing::info;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Item {
    pub account_type: String,
    pub age: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Clone)]
pub struct ModelManager {
    table_name: String,
    db_client: Client,
}
impl ModelManager {
    pub fn new(table_name: &str, config: &SdkConfig) -> Self {
        ModelManager {
            db_client: Client::new(config),
            table_name: table_name.into(),
        }
    }
}

impl ModelManager {
    pub async fn scan(&self) -> Result<Vec<Item>, Error> {
        let result = self
            .db_client
            .scan()
            .table_name(&self.table_name)
            .send()
            .await?;

        let items_result = result.items().to_vec();
        let items: Vec<Item> = from_items(items_result)?;
        Ok(items)
    }

    pub async fn put(&self, item: Item) -> Result<(), Error> {
        let user_av = to_attribute_value(item.username)?;
        let type_av = to_attribute_value(item.account_type)?;
        let age_av = to_attribute_value(item.age)?;
        let first_av = to_attribute_value(item.first_name)?;
        let last_av = to_attribute_value(item.last_name)?;

        let request = self
            .db_client
            .put_item()
            .table_name(&self.table_name)
            .item("username", user_av)
            .item("account_type", type_av)
            .item("age", age_av)
            .item("first_name", first_av)
            .item("last_name", last_av);

        info!("adding item to DynamoDB");

        let _resp = request.send().await?;

        Ok(())
    }
}
