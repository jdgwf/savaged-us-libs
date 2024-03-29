use chrono::prelude::*;
use serde::{Deserialize, Serialize};
// use serde::de::{self, Unexpected};
// use chrono_tz::Tz;

use super::utils::bool_from_int_or_bool;
use super::utils::float_to_int;
use crate::public_user_info::PublicUserInfo;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Banner {
    pub id: u32,
    #[serde(default)]
    pub end: Option<DateTime<Utc>>,
    pub href: String,
    pub name: String,
    // pub r#type: String,
    pub image: String,
    #[serde(default)]
    pub start: Option<DateTime<Utc>>,
    pub title: String,
    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub active: bool,

    #[serde(deserialize_with = "float_to_int", default)]
    pub clicks: u32,

    // #[serde(deserialize_with = "bool_from_int_or_bool")]
    // pub deleted: bool,
    // pub summary: String,
    // #[serde(deserialize_with = "bool_from_int_or_bool")]
    // pub read_only: bool,
    pub created_by: u32,
    #[serde(default)]
    pub created_on: Option<DateTime<Utc>>,
    pub deleted_by: u32,
    #[serde(default)]
    pub deleted_on: Option<DateTime<Utc>>,
    pub updated_by: u32,
    #[serde(default)]
    pub updated_on: Option<DateTime<Utc>>,
    // pub version_of: u32,
    // pub description: String,
    #[serde(deserialize_with = "float_to_int", default)]
    pub impressions: u32,
    // pub name_plural: String,
    #[serde(deserialize_with = "bool_from_int_or_bool", default)]
    pub prevent_hiding: bool,

    #[serde(default)]
    pub created_by_obj: Option<PublicUserInfo>,

    #[serde(default)]
    pub updated_by_obj: Option<PublicUserInfo>,

    #[serde(default)]
    pub deleted_by_obj: Option<PublicUserInfo>,
}

impl Default for Banner {
    fn default() -> Self {
        Banner {
            id: 0,
            end: None,
            href: "".to_owned(),
            name: "".to_owned(),
            // r#type: "".to_owned(),
            image: "".to_owned(),
            start: None,
            title: "".to_owned(),
            active: false,
            clicks: 0,
            // deleted: false,
            // summary: "".to_owned(),
            // read_only: false,
            created_by: 0,
            created_on: Some(chrono::offset::Utc::now()),
            deleted_by: 0,
            deleted_on: None,
            updated_by: 0,
            updated_on: None,
            // version_of: 0,
            // description:  "".to_owned(),
            impressions: 0,
            // name_plural: "".to_owned(),
            prevent_hiding: false,

            created_by_obj: None,
            deleted_by_obj: None,
            updated_by_obj: None,
        }
    }
}

impl Banner {
    pub fn to_simple(&self) -> SimpleBanner {
        SimpleBanner {
            id: self.id,
            href: self.href.to_owned(),
            name: self.name.to_owned(),
            image: self.image.to_owned(),
            title: self.title.to_owned(),
            prevent_hiding: self.prevent_hiding,
        }
    }
}

#[derive(Serialize, PartialEq, Deserialize, Clone, Debug)]
pub struct SimpleBanner {
    pub id: u32,
    pub href: String,
    pub name: String,
    pub image: String,
    pub title: String,
    pub prevent_hiding: bool,

}
