use chrono::prelude::*;
use serde::{Deserialize, Serialize};
// use serde::de::{self, Unexpected};
// use chrono_tz::Tz;

use crate::public_user_info::PublicUserInfo;

use super::utils::bool_from_int_or_bool;
use super::utils::float_to_int;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Partner {

    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub ace: bool,
    pub company: String,
    pub contact_email: String,
    pub contact_name: String,
    // logo: String,
    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub swag: bool,
    pub text: String,
    pub url: String,
    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub use_permission: bool,

    pub id: u32,
    // #[serde(default)]
    // pub end: Option<DateTime<Utc>>,
    // pub href: String,
    // pub name: String,
    // pub r#type: String,
    #[serde(default, alias="logo")]
    pub image: String,
    #[serde(default)]
    pub date: Option<DateTime<Utc>>,
    // pub title: String,
    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub active: bool,

    // #[serde(deserialize_with = "float_to_int", default)]
    // pub clicks: u32,

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

    #[serde(default)]
    pub description: String,

    // #[serde(deserialize_with = "float_to_int", default)]
    // pub impressions: u32,
    pub name_plural: String,
    // #[serde(deserialize_with = "bool_from_int_or_bool", default)]
    // pub prevent_hiding: bool,

    #[serde(default)]
    pub created_by_obj: Option<PublicUserInfo>,

    #[serde(default)]
    pub updated_by_obj: Option<PublicUserInfo>,

    #[serde(default)]
    pub deleted_by_obj: Option<PublicUserInfo>,
}

impl Default for Partner {
    fn default() -> Self {
        Partner {
            id: 0,
            // end: None,
            // href: "".to_owned(),
            // name: "".to_owned(),
            // r#type: "".to_owned(),
            image: "".to_owned(),
            date: None,
            // title: "".to_owned(),
            active: false,
            // clicks: 0,
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
            description: "".to_owned(),
            // impressions: 0,
            name_plural: "".to_owned(),
            // prevent_hiding: false,

            created_by_obj: None,
            deleted_by_obj: None,
            updated_by_obj: None,

            ace: false,
            company: "".to_owned(),
            contact_email: "".to_owned(),
            contact_name: "".to_owned(),
            swag: false,
            text: "".to_owned(),
            url: "".to_owned(),
            use_permission: false,
        }
    }
}

#[derive(Serialize, PartialEq, Deserialize, Clone, Debug)]
pub struct SimplePartner {
    pub id: u32,
    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub ace: bool,
    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub swag: bool,
    // pub href: String,
    pub company: String,

    #[serde(default, alias="logo")]
    pub image: String,
    // pub title: String,
    // pub summary: String,
    // pub description: String,

}