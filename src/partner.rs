use serde::{Serialize, Deserialize};
use chrono::prelude::*;
// use serde::de::{self, Unexpected};
// use chrono_tz::Tz;

use super::utils::float_to_int;
use super::utils::bool_from_int_or_bool;

#[derive(Serialize, Deserialize, Clone, Eq, Debug, PartialEq)]
pub struct Partner {
    pub id: u32,
    #[serde(default)]
    pub end: Option<DateTime<Utc>>,
    pub href: String,
    pub name: String,
    pub r#type: String,
    pub image: String,
    #[serde(default)]
    pub start: Option<DateTime<Utc>>,
    pub title: String,
    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub active: bool,

    #[serde(deserialize_with = "float_to_int",default)]
    pub clicks: u32,

    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub deleted: bool,
    pub summary: String,
    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub read_only: bool,
    pub created_by: u32,
    #[serde(default)]
    pub created_on: Option<DateTime<Utc>>,
    pub deleted_by: u32,
    #[serde(default)]
    pub deleted_on: Option<DateTime<Utc>>,
    pub updated_by: u32,
    #[serde(default)]
    pub updated_on: Option<DateTime<Utc>>,
    pub version_of: u32,
    pub description: String,

    #[serde(deserialize_with = "float_to_int",default)]
    pub impressions: u32,
    pub name_plural: String,
    #[serde(deserialize_with = "bool_from_int_or_bool",default)]
    pub prevent_hiding: bool,
    // pub created_by_user,
    // pub deleted_by_user,
    // pub updated_by_user,
}

impl Default for Partner {

    fn default() -> Self {
        Partner {
            id: 0,
            end: None,
            href: "".to_owned(),
            name: "".to_owned(),
            r#type: "".to_owned(),
            image: "".to_owned(),
            start: None,
            title: "".to_owned(),
            active: false,
            clicks: 0,
            deleted: false,
            summary: "".to_owned(),
            read_only: false,
            created_by: 0,
            created_on: Some(chrono::offset::Utc::now()),
            deleted_by: 0,
            deleted_on: None,
            updated_by: 0,
            updated_on: None,
            version_of: 0,
            description:  "".to_owned(),
            impressions: 0,
            name_plural: "".to_owned(),
            prevent_hiding: false,
        }
    }
}

