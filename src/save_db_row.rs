use serde::{Serialize, Deserialize};
use crate::public_user_info::PublicUserInfo;
use chrono::prelude::*;

use super::utils::bool_from_int_or_bool;
use super::utils::deserialize_null_default;
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct SaveDBRow {
    pub id: i64,

    // #[serde(default)]
    // pub session_id: i64,

    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub name: String,

    pub sort_order: i64,

    #[serde(default)]
    pub save_type: String,
    // #[serde(default)]
    pub export_generic_json: String,

    // user_is_wildcard?: bool,

    // #[serde(default)]
    // setting_name: String,

    #[serde(default)]
    pub shareurl: String,
    #[serde(default)]
    pub short_desc: String,

    pub share_public: bool ,
    pub share_copy: bool,

    // #[serde(default)]
    pub imageurl: String,
    // #[serde(default)]
    // imagetokenurl: String,
    // #[serde(default)]
    // imagesettingurl: String,
    // #[serde(default)]
    pub folder: String,

    pub created_by: i64,

    pub rifts_living_campaign: bool,

    pub updated_by: i64,

    // #[serde(default)]
    pub share_html: String,

    pub hits: i64,
    pub total_hits: i64,

    #[serde(default)]
    pub data: String,

    pub deleted_by: i64,

    pub show_character_sheet: bool,
    pub allow_download: bool,

    pub session_id: i64,
    pub co_owner: i64,
    // #[serde(default)]
    pub co_owner_folder: String,
    // co_owner_public: IPublicUserInfo,

    #[serde(default)]
    pub created_on: Option<DateTime<Utc>>,
    #[serde(default)]
    pub updated_on: Option<DateTime<Utc>>,
    #[serde(default)]
    pub deleted_on: Option<DateTime<Utc>>,
    #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub deleted: bool,

    pub created_by_public: Option<PublicUserInfo>,
    pub updated_by_public: Option<PublicUserInfo>,
    pub deleted_by_public: Option<PublicUserInfo>,
    pub co_owner_public: Option<PublicUserInfo>,
}