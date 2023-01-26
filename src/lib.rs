pub mod admin_libs;
pub mod alert_level;
pub mod banner;
pub mod book;
pub mod game_data_row;
pub mod hidden_banner;
pub mod notification;
pub mod partner;
pub mod player_character;
pub mod public_user_info;
pub mod save_db_row;
pub mod setting;
pub mod user;
pub mod utils;
pub mod websocket_message;
use serde::{Deserialize, Deserializer};

pub fn clean_json_data( send_data: String ) -> String {

    let mut rv = send_data.clone();
    rv = rv.replace("\\\"pf_armor_type\\\": \\\"Armor\\\",", "");
    rv = rv.replace("\"pf_armor_type\": \"Armor\",", "");
    rv = rv.replace("\\\"pf_armor_type\\\":\\\"Armor\\\",", "");
    rv = rv.replace("\"pf_armor_type\":\"Armor\",", "");

    rv = rv.replace(",\\\"pf_armor_type\\\": \\\"Armor\\\"", "");
    rv = rv.replace(",\"pf_armor_type\": \"Armor\"", "");
    rv = rv.replace(",\\\"pf_armor_type\\\":\\\"Armor\\\"", "");
    rv = rv.replace(",\"pf_armor_type\":\"Armor\"", "");
    rv = rv.replace(", \\\"pf_armor_type\\\": \\\"Armor\\\"", "");
    rv = rv.replace(", \"pf_armor_type\": \"Armor\"", "");
    rv = rv.replace(", \\\"pf_armor_type\\\":\\\"Armor\\\"", "");
    rv = rv.replace(", \"pf_armor_type\":\"Armor\"", "");

    return rv;
}

pub fn serde_string_to_u32_default_0<'de, D>(deserializer: D) -> Result<u32, D::Error> where D: Deserializer<'de> {
    let s = String::deserialize(deserializer)?;
    // u32::from_str(&s).map_err(de::Error::custom)
    if s.is_empty() {
        return Ok(0);
    } else {
        return Ok(s.as_str().parse::<u32>().unwrap_or(0));
    }
}