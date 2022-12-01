use serde::{Serialize, Deserialize};
use super::super::utils::bool_from_int_or_bool;
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct JSONSaveDBRow {
    id: i64,

    #[serde(default)]
    campaign_id: i64,
    #[serde(default)]
    pub name: String,

    sort_order: i64,

    #[serde(default)]
    pub save_type: String,
    // #[serde(default)]
    // export_generic_json: String,

    // user_is_wildcard?: bool,

    // #[serde(default)]
    // setting_name: String,

    // #[serde(default)]
    // shareurl: String,
    // #[serde(default)]
    // short_desc: String,

    // share_public: bool ,
    // share_copy: bool,

    // #[serde(default)]
    // imageurl: String,
    // #[serde(default)]
    // imagetokenurl: String,
    // #[serde(default)]
    // imagesettingurl: String,
    // #[serde(default)]
    // folder: String,

    created_by: i64,

    // rifts_living_campaign: bool,

    updated_by: i64,

    // #[serde(default)]
    // share_html: String,

    hits: i64,
    total_hits: i64,

    #[serde(default)]
    pub data: String,

    deleted_by: i64,

    // show_character_sheet: bool,
    // allow_download: bool,

    session_id: i64,
    co_owner: i64,
    // #[serde(default)]
    // co_owner_folder: String,
    // co_owner_public: IPublicUserInfo,

    #[serde(default)]
    pub created_on: String,
    #[serde(default)]
    pub updated_on: String,
    #[serde(default)]
    pub deleted_on: String,
    #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub deleted: bool,
}