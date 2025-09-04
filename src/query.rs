use serde::{Serialize, Deserialize};
use time::OffsetDateTime;

#[cfg(feature = "validate")]
use validator::Validate;

fn default_before() -> OffsetDateTime {
    OffsetDateTime::now_utc()
}
fn default_limit() -> i64 {
    25
}

#[cfg_attr(feature = "validate", derive(validator::Validate))]
#[derive(Deserialize, Serialize, Debug)]
pub struct ListQuery {
    #[serde(with = "time::serde::rfc3339")]
    #[serde(default = "default_before")]
    pub before: OffsetDateTime,
    #[serde(default = "default_limit")]
    #[cfg_attr(feature = "validate", validate(min = 0, max = 200))]
    pub limit: i64,
}
