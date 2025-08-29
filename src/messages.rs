use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[cfg(feature = "validate")]
use validator::Validate;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reaction {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub reaction: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReaction {
    pub user_id: Uuid,
    pub reaction: String,
}

#[cfg_attr(feature = "db", derive(sqlx::FromRow))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub author_id: Option<Uuid>,
    pub content: String,
    pub media: Vec<Uuid>,
    pub is_edited: bool,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    pub reactions: Vec<Reaction>,
}


#[cfg_attr(feature = "validate", derive(validator::Validate))]
#[derive(Deserialize, Debug)]
pub struct CreateMessage {
    pub id: Uuid,
    pub author_id: Uuid,
    #[cfg_attr(feature = "validate", validate(length(min = 1)))]
    pub content: String,
    pub media: Vec<Uuid>,
}

#[cfg_attr(feature = "validate", derive(validator::Validate))]
#[derive(Deserialize, Debug)]
pub struct UpdateMessage {
    pub author_id: Option<Uuid>,
    #[cfg_attr(feature = "validate", validate(length(min = 1)))]
    pub content: Option<String>,
    pub media: Option<Vec<Uuid>>
}

