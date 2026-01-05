use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
#[repr(transparent)]
#[schema(value_type = String)]
pub struct UserId(uuid::Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }

    pub fn from_uuid(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> uuid::Uuid {
        self.0
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<uuid::Uuid> for UserId {
    fn from(uuid: uuid::Uuid) -> Self {
        Self(uuid)
    }
}

impl From<UserId> for uuid::Uuid {
    fn from(user_id: UserId) -> Self {
        user_id.0
    }
}

impl std::fmt::Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}
