use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use ipnetwork::IpNetwork;
use uuid::Uuid;


#[cfg_attr(feature = "db", derive(sqlx::Type))]
#[cfg_attr(feature = "db", sqlx(type_name = "revokereason", rename_all = "snake_case"))]
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RevokeReason {
    Logout,
    Expired,
    TokenReused,
    PasswordChanged,
    AdminLogout,
    UserDeleted,
}
impl RevokeReason {
    pub fn to_string(&self) -> String {
        match self {
            RevokeReason::Logout => "logout".to_string(),
            RevokeReason::Expired => "expired".to_string(),
            RevokeReason::TokenReused => "token_reused".to_string(),
            RevokeReason::PasswordChanged => "password_changed".to_string(),
            RevokeReason::AdminLogout => "admin_logout".to_string(),
            RevokeReason::UserDeleted => "user_deleted".to_string(),
        }
    }
}

#[cfg_attr(feature = "db", derive(sqlx::FromRow))]
#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub id: Uuid,
    pub user_id: Uuid,
    pub hashed_token: String,
    pub is_revoked: bool,
    #[serde(with = "time::serde::rfc3339::option")]
    pub revoked_at: Option<OffsetDateTime>,
    pub revoke_reason: Option<RevokeReason>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub expires_at: OffsetDateTime,
    pub ip_address: Option<IpNetwork>,
}

#[derive(Deserialize, Debug)]
pub struct CreateSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub hashed_token: String,
    #[serde(with = "time::serde::rfc3339")]
    pub expires_at: OffsetDateTime,
    pub ip_address: Option<IpNetwork>,
}

