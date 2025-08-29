use serde::{Deserialize, Deserializer};

// This enum allows null values to be specified as fields but be treated differently to missing
// fields. This is achieved by a defaulting to a `Missing` enum value, reserving `Null` for truely
// null values.

#[derive(Debug, Clone, PartialEq)]
pub enum NullableField<T> {
    Missing,     // Field not provided in JSON
    Null,        // Field provided as null
    Value(T),    // Field provided with a value
}

impl<T> Default for NullableField<T> {
    fn default() -> Self {
        NullableField::Missing
    }
}

impl<T> NullableField<T> {
    pub fn is_missing(&self) -> bool {
        matches!(self, NullableField::Missing)
    }

    pub fn is_null(&self) -> bool {
        matches!(self, NullableField::Null)
    }

    pub fn is_value(&self) -> bool {
        matches!(self, NullableField::Value(_))
    }

    pub fn into_option(self) -> Option<T> {
        match self {
            NullableField::Value(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_option(&self) -> Option<&T> {
        match self {
            NullableField::Value(v) => Some(v),
            _ => None,
        }
    }

    // Convert to database-compatible Option<T> where Missing preserves existing value
    pub fn to_db_option(self) -> Option<Option<T>> {
        match self {
            NullableField::Missing => None,           // Don't update
            NullableField::Null => Some(None),        // Set to NULL
            NullableField::Value(v) => Some(Some(v)), // Set to value
        }
    }
}

// Custom Serde implementation
impl<'de, T> Deserialize<'de> for NullableField<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let opt = Option::<T>::deserialize(deserializer)?;
        Ok(match opt {
            Some(value) => NullableField::Value(value),
            None => NullableField::Null,
        })
    }
}

