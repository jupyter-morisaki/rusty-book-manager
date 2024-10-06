use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

use shared::error::AppError;

macro_rules! define_id {
    ($id_type: ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize, sqlx::Type)]
        #[serde(into = "String")]
        #[sqlx(transparent)]
        pub struct $id_type(Uuid);

        impl $id_type {
            pub fn new() -> Self {
                Self(Uuid::new_v4())
            }

            pub fn raw(self) -> Uuid {
                self.0
            }
        }

        impl Default for $id_type {
            fn default() -> Self {
                Self::new()
            }
        }

        impl FromStr for $id_type {
            type Err = AppError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(Self(Uuid::parse_str(s)?))
            }
        }

        impl From<Uuid> for $id_type {
            fn from(id: Uuid) -> Self {
                Self(id)
            }
        }

        impl Display for $id_type {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(
                    f,
                    "{}",
                    self.0.as_simple().encode_lower(&mut Uuid::encode_buffer())
                )
            }
        }

        impl From<$id_type> for String {
            fn from(id: $id_type) -> Self {
                id.to_string()
            }
        }
    };
}

define_id!(BookId);
