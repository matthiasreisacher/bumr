use std::collections::HashMap;
use std::convert::TryFrom;

use anyhow::{Context, Result};
use rusoto_dynamodb::{AttributeValue, DeleteItemInput, DynamoDb, DynamoDbClient, GetItemInput,
                      PutItemInput};

use crate::database::dynamodb::{i64_to_attribute_number, string_to_attribute_string};
use crate::model::session::Session;

static TABLE_NAME: &str = "BumrSession";

static ATTRIBUTE_ID: &str = "Id";
static ATTRIBUTE_USER_ID: &str = "UserId";
static ATTRIBUTE_CREATED_AT: &str = "CreatedAt";

impl TryFrom<HashMap<String, AttributeValue>> for Session {
    type Error = anyhow::Error;

    fn try_from(map: HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        Ok(Session {
            id: map.get(ATTRIBUTE_ID).context(format!("No session property '{}'", ATTRIBUTE_ID))?
                .s.as_deref().context(format!("Empty session property '{}'", ATTRIBUTE_ID))?
                .to_owned(),
            user_id: map.get(ATTRIBUTE_USER_ID)
                .context(format!("No session property '{}'", ATTRIBUTE_USER_ID))?
                .s.as_deref().context(format!("Empty session property '{}'", ATTRIBUTE_USER_ID))?
                .to_owned(),
            created_at: map.get(ATTRIBUTE_CREATED_AT)
                .context(format!("No session property '{}'", ATTRIBUTE_CREATED_AT))?
                .n.as_deref().context(format!("Empty session property '{}'", ATTRIBUTE_CREATED_AT))?
                .parse::<i64>()
                .context(format!("Session property '{}' not parsable", ATTRIBUTE_CREATED_AT))?,
        })
    }
}


impl Session {
    /// Creates the input for a DynamoDB Get-Item request for a session.
    pub fn get_item(id: &str) -> GetItemInput {
        let mut attributes: HashMap<String, AttributeValue> = HashMap::new();
        attributes.insert(ATTRIBUTE_ID.to_owned(), string_to_attribute_string(id));

        let mut item = GetItemInput::default();
        item.table_name = TABLE_NAME.to_owned();
        item.key = attributes;

        item
    }

    /// Creates the input for a DynamoDB Put-Item request from the session.
    pub fn put_item(&self) -> PutItemInput {
        let mut attributes: HashMap<String, AttributeValue> = HashMap::new();
        attributes.insert(ATTRIBUTE_ID.to_owned(), string_to_attribute_string(&self.id));
        attributes.insert(ATTRIBUTE_USER_ID.to_owned(), string_to_attribute_string(&self.user_id));
        attributes.insert(ATTRIBUTE_CREATED_AT.to_owned(), i64_to_attribute_number(self.created_at));

        let mut item = PutItemInput::default();
        item.table_name = TABLE_NAME.to_owned();
        item.item = attributes;

        item
    }

    /// Creates the input for a DynamoDB Delete-Item request for a session.
    pub fn delete_item(id: &str) -> DeleteItemInput {
        let mut attributes: HashMap<String, AttributeValue> = HashMap::new();
        attributes.insert(ATTRIBUTE_ID.to_owned(), string_to_attribute_string(id));

        let mut item = DeleteItemInput::default();
        item.table_name = TABLE_NAME.to_owned();
        item.key = attributes;

        item
    }
}


/// Fetches the session with the given id.
pub async fn get(client: &DynamoDbClient, id: &str) -> Result<Option<Session>> {
    match client.get_item(Session::get_item(id)).await {
        Ok(output) => match output.item {
            Some(session) => {
                let session = Session::try_from(session)?;
                return Ok(Some(session));
            }
            None => info!("Could not find session with id {}", id),
        }
        Err(error) => bail!("{}", error),
    }
    Ok(None)
}

/// Creates a new session, or replaces and existing one with the given session.
pub async fn put(client: &DynamoDbClient, session: &Session) -> Result<()> {
    client
        .put_item(session.put_item()).await
        .and_then(|_| {
            debug!("Created session {}", session.id);
            Ok(())
        })
        .or_else(|error| bail!("{}", error))
}

/// Deletes the session with the given id.
pub async fn delete(client: &DynamoDbClient, id: &str) -> Result<()> {
    client
        .delete_item(Session::delete_item(id)).await
        .and_then(|_| {
            debug!("Deleted session {}", id);
            Ok(())
        })
        .or_else(|error| bail!("{}", error))
}
