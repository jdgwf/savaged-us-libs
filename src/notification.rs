use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Notification {
    pub id: u32,
    pub user_id: u32,
    pub read: u32,
    pub subject: String,
    pub message: String,
    pub created_by: u32,
    #[serde(default)]
    pub created_on: Option<DateTime<Utc>>,
}
