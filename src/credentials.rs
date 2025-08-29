use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[cfg(feature = "validate")]
use validator::Validate;

use super::users::{User, CreateUser};

// -- CREDENTIALS --

#[cfg_attr(feature = "db", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub user_id: Uuid,
    pub encrypted_email_address: String,
    pub hashed_email_address: String,
    pub hashed_password: String,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[cfg_attr(feature = "validate", derive(validator::Validate))]
#[derive(Deserialize, Debug)]
pub struct SecureEmailAddress {
    pub encrypted_email_address: String,
    #[cfg_attr(feature = "validate", validate(length(min = 43, max = 43)))]
    pub hashed_email_address: String,
}

#[cfg_attr(feature = "validate", derive(validator::Validate))]
#[derive(Deserialize, Debug)]
pub struct CreateCredentials {
    pub user_id: Uuid,
    #[cfg_attr(feature = "validate", validate(nested))]
    pub email_address: SecureEmailAddress,
    pub hashed_password: String,
}

#[cfg_attr(feature = "validate", derive(validator::Validate))]
#[derive(Deserialize, Debug)]
pub struct CreateCredentialsAndUser {
    pub user: CreateUser,
    #[cfg_attr(feature = "validate", validate(nested))]
    pub email_address: SecureEmailAddress,
    pub hashed_password: String,
}

#[derive(Serialize, Debug)]
pub struct CredentialsAndUser {
    pub credentials: Credentials,
    pub user: User 
}

#[cfg_attr(feature = "validate", derive(validator::Validate))]
#[derive(Deserialize, Debug)]
pub struct UpdateCredentials {
    #[cfg_attr(feature = "validate", validate(nested))]
    pub email_address: Option<SecureEmailAddress>,
    pub hashed_password: Option<String>,
}


