use serde::{Serialize, Deserialize, Serializer, Deserializer};
use chrono::prelude::*;
use serde::de::{self, Unexpected};
use chrono_tz::Tz;

use super::utils::float_to_int;
use super::utils::bool_from_int_or_bool;
use super::utils::deserialize_null_default;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Book {
    #[serde(default)]
    pub id: u32,

    // #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub image: String,

    #[serde(default)]
    pub logo: String,

    // #[serde(default)]
    pub publisher: String,

    // #[serde(default)]
    pub published: String,

    // #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub active: bool,

    // #[serde(default, deserialize_with = "bool_from_int_or_bool")]

    #[serde(default)]
    pub deleted: bool,

    // #[serde(default)]
    pub summary: String,

    // #[serde(default)]
    pub created_by: u32,

    // #[serde(default)]
    pub created_on: Option<DateTime<Utc>>,

    // #[serde(default)]
    pub deleted_by: u32,

    // #[serde(default)]
    pub deleted_on: Option<DateTime<Utc>>,

    // #[serde(default)]
    pub updated_by: u32,

    // #[serde(default)]
    pub updated_on: Option<DateTime<Utc>>,

    // #[serde(default)]
    pub version_of: u32,

    // #[serde(default)]
    pub description: Vec<String>,


    // #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub access_registered: bool,
    // #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub access_wildcard: bool,
    // #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub access_developer: bool,
    // #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub access_admin: bool,
    // #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub access_anonymous: bool,
}

impl Default for Book {

    fn default() -> Self {
        Book {
            id: 0,

            name: "".to_owned(),

            image: "".to_owned(),
            logo: "".to_owned(),

            publisher: "".to_owned(),
            published: "".to_owned(),
            active: false,

            deleted: false,
            summary: "".to_owned(),

            created_by: 0,
            created_on: Some(chrono::offset::Utc::now()),
            deleted_by: 0,
            deleted_on: None,
            updated_by: 0,
            updated_on: None,
            version_of: 0,
            description: Vec::new(),

            access_registered: false,
            access_wildcard: false,
            access_developer: false,
            access_admin: false,
            access_anonymous: false,
        }
    }
}

