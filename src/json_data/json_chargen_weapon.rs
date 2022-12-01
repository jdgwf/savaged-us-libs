use serde::{Serialize, Deserialize};
use super::super::utils::bool_from_int_or_bool;
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct JSONWeaponDefinition {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub summary: String,

    pub book_id: u32,
    #[serde(default)]
    pub page: String,

    #[serde(default)]
    pub created_on: String,
    #[serde(default)]
    pub updated_on: String,
    #[serde(default)]
    pub deleted_on: String,
    #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub deleted: bool,

}