use serde::{Deserialize, Serialize};
use uuid::Uuid;
use time::OffsetDateTime;

#[cfg(feature = "validate")]
use validator::Validate;

use crate::{enums::NullableField, users::User};


#[cfg(feature = "validate")]
mod validator_fns {
    use validator::ValidationError;
    use std::collections::HashSet;
    use log::debug;

    use super::{CreateChat, UpdateChat};

    pub fn validate_owner_is_participant(chat: &CreateChat) -> Result<(), ValidationError> {
        if !chat.participants.contains(&chat.owner_id) {
            let mut err = ValidationError::new("owner_is_not_participant");
            err.message = Some("owner_id must be included in participants".into());
            debug!("[CLIENT ERROR]: {}", err);
            return Err(err);
        }
        Ok(())
    }

    pub fn validate_owner_is_not_removed(chat: &UpdateChat) -> Result<(), ValidationError> {
        if let Some(remove_participants) = &chat.remove_participants {
            if let Some(owner_id) = chat.owner_id {
                if remove_participants.contains(&owner_id) {
                    let mut err = ValidationError::new("owner_id_in_remove_participants");
                    err.message =
                        Some("provided owner_id must not be included in remove_participants".into());
                    debug!("[CLIENT ERROR]: {}", err);
                    return Err(err);
                }
            }
        }
        Ok(())
    }

    fn has_common<T: Eq + std::hash::Hash>(a: &[T], b: &[T]) -> bool {
        let set: HashSet<_> = a.iter().collect();
        b.iter().any(|item| set.contains(item))
    }

    pub fn validate_no_participant_conflicts(chat: &UpdateChat) -> Result<(), ValidationError> {
        if let Some(remove_participants) = &chat.remove_participants {
            if let Some(add_participants) = &chat.add_participants {
                if has_common(add_participants, remove_participants) {
                    let mut err = ValidationError::new("participant_conflicts");
                    err.message =
                        Some("add_participants and remove_participants contain conflicts".into());
                    debug!("[CLIENT ERROR]: {}", err);
                    return Err(err);
                }
            }
        }
        Ok(())
    }
}


#[cfg_attr(feature = "db", derive(sqlx::FromRow, sqlx::Decode))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Chat {
    pub id: Uuid,
    pub owner_id: Option<Uuid>,
    pub name: Option<String>,
    pub is_group: bool,
    pub participants: Vec<User>,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
}

#[cfg_attr(feature = "validate", derive(validator::Validate))]
#[cfg_attr(feature = "validate", validate(schema(
    function = "validator_fns::validate_owner_is_participant",
    skip_on_field_errors = false
)))]
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateChat {
    pub id: Uuid,
    pub name: Option<String>,
    pub owner_id: Uuid,
    pub is_group: bool,
    #[cfg_attr(feature = "validate", validate(length(min = 2)))]
    pub participants: Vec<Uuid>,
}


#[cfg_attr(feature = "validate", derive(validator::Validate))]
#[cfg_attr(feature = "validate", validate(
    schema(
        function = "validator_fns::validate_owner_is_not_removed",
        skip_on_field_errors = false
    ),
    schema(
        function = "validator_fns::validate_no_participant_conflicts",
        skip_on_field_errors = false
    )
))]
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateChat {
    #[serde(default, skip_serializing_if = "NullableField::is_missing")]
    pub name: NullableField<String>,
    pub owner_id: Option<Uuid>,
    pub add_participants: Option<Vec<Uuid>>,
    pub remove_participants: Option<Vec<Uuid>>,
}
