use anyhow::Result;
use async_trait::async_trait;
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDbClient, DynamoDb, ListTablesInput};

use crate::config::Config;
use crate::database::Database;
use crate::model::session::Session;
use futures::executor::block_on;

pub mod session;

pub struct DynamoDbDatabase {
    client: DynamoDbClient
}

impl DynamoDbDatabase {
    pub fn new(name: &str, endpoint: &str) -> Self {
        let client = DynamoDbClient::new(Region::Custom {
            name: name.to_owned(),
            endpoint: endpoint.to_owned(),
        });

        DynamoDbDatabase { client }
    }

    pub fn validate(&self) -> Result<()> {
        block_on(
            async {
                self.client.list_tables(ListTablesInput::default()).await
                    .map(|_| ())
                    .map_err(anyhow::Error::msg)
            }
        )
    }
}

#[async_trait]
impl Database for DynamoDbDatabase {
    async fn session_get(&self, id: &str) -> Result<Option<Session>> {
        session::get(&self.client, id).await
    }

    async fn session_put(&self, session: &Session) -> Result<()> {
        session::put(&self.client, session).await
    }

    async fn session_delete(&self, id: &str) -> Result<()> {
        session::delete(&self.client, id).await
    }
}

/// Creates, initializes and returns a new DynamoDB database object.
pub fn init(config: &Config) -> Box<dyn Database> {
    let db = DynamoDbDatabase::new(
        &config.aws.region,
        &config.aws.dynamodb.endpoint_url,
    );
    Box::new(db)
}

/// Helper function to create a string-attribute of the given value.
fn string_to_attribute_string(val: &str) -> AttributeValue {
    let mut att = AttributeValue::default();
    att.s = Some(val.to_string());
    att
}

/// Helper function to create a number-attribute of the given value.
fn i64_to_attribute_number(val: i64) -> AttributeValue {
    let mut att = AttributeValue::default();
    att.n = Some(val.to_string());
    att
}