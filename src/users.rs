use serde::{Serialize, Deserialize};
use time::OffsetDateTime;

use uuid::Uuid;

use crate::enums::NullableField;


#[cfg_attr(feature = "db", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub profile_picture_url: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

// Are these necessary?

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUser {
    pub id: Uuid,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub profile_picture_url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUser {
    pub username: Option<String>,
    #[serde(default)]
    pub display_name: NullableField<String>,
    #[serde(default)]
    pub bio: NullableField<String>,
    #[serde(default)]
    pub profile_picture_url: NullableField<String>,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct UsersBatchResult {
    pub users: Vec<User>,
    pub not_found: Vec<Uuid>,
}

