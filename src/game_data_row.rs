use crate::{
    player_character::{
        armor::Armor, edge::Edge, gear::Gear, hindrance::Hindrance, weapon::Weapon, gear_enhancement::GearEnhancement,
    },
    public_user_info::PublicUserInfo,
};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameDataRow {
    pub active: bool,
    pub book_id: u32,
    pub id: u32,
    pub created_by: u32,

    #[serde(default)]
    pub created_on: Option<DateTime<Utc>>,

    pub deleted: bool,
    pub deleted_by: u32,

    #[serde(default)]
    pub deleted_on: Option<DateTime<Utc>>,

    pub data: String,
    pub name: String,

    pub updated_by: u32,

    #[serde(default)]
    pub updated_on: Option<DateTime<Utc>>,

    pub version_of: u32,

    #[serde(default)]
    pub created_by_user: Option<PublicUserInfo>,
    #[serde(default)]
    pub updated_by_user: Option<PublicUserInfo>,
    #[serde(default)]
    pub deleted_by_user: Option<PublicUserInfo>,

    #[serde(default)]
    pub book_name: Option<String>,
    pub book_short_name: Option<String>,
}

impl GameDataRow {
    pub fn to_hindrance(&self) -> Result<Hindrance, serde_json::Error> {
        let item_result: Result<Hindrance, serde_json::Error> =
            serde_json::from_str(self.data.as_str());
        match item_result {
            Ok(mut item) => {
                item.book_name = self.book_name.clone();
                item.book_short_name = self.book_short_name.clone();

                item.id = self.id;
                item.active = self.active;
                item.created_by = self.created_by;
                item.deleted_by = self.deleted_by;
                item.updated_by = self.updated_by;

                item.created_on = self.created_on.clone();
                item.deleted_on = self.deleted_on.clone();
                item.updated_on = self.updated_on.clone();

                item.created_by_obj = self.created_by_user.clone();
                item.deleted_by_obj = self.deleted_by_user.clone();
                item.updated_by_obj = self.updated_by_user.clone();

                return Ok(item);
            }
            Err(err) => {
                println!("{}", self.data);
                return Err(err);
            }
        }
    }

    pub fn to_edge(&self) -> Result<Edge, serde_json::Error> {
        let item_result: Result<Edge, serde_json::Error> = serde_json::from_str(self.data.as_str());
        match item_result {
            Ok(mut item) => {
                item.book_name = self.book_name.clone();
                item.book_short_name = self.book_short_name.clone();

                item.id = self.id;
                item.active = self.active;
                item.created_by = self.created_by;
                item.deleted_by = self.deleted_by;
                item.updated_by = self.updated_by;

                item.created_on = self.created_on.clone();
                item.deleted_on = self.deleted_on.clone();
                item.updated_on = self.updated_on.clone();

                item.created_by_obj = self.created_by_user.clone();
                item.deleted_by_obj = self.deleted_by_user.clone();
                item.updated_by_obj = self.updated_by_user.clone();

                return Ok(item);
            }
            Err(err) => {
                println!("{}", self.data);
                return Err(err);
            }
        }
    }

    pub fn to_gear(&self) -> Result<Gear, serde_json::Error> {

        let item_result: Result<Gear, serde_json::Error> = serde_json::from_str(self.data.as_str());
        match item_result {
            Ok(mut item) => {
                item.book_name = self.book_name.clone();
                item.book_short_name = self.book_short_name.clone();

                item.id = self.id;
                item.active = self.active;
                item.created_by = self.created_by;
                item.deleted_by = self.deleted_by;
                item.updated_by = self.updated_by;

                item.created_on = self.created_on.clone();
                item.deleted_on = self.deleted_on.clone();
                item.updated_on = self.updated_on.clone();

                item.created_by_obj = self.created_by_user.clone();
                item.deleted_by_obj = self.deleted_by_user.clone();
                item.updated_by_obj = self.updated_by_user.clone();

                return Ok(item);
            }
            Err(err) => {
                println!("{}", self.data);
                return Err(err);
            }
        }
    }

    pub fn to_weapon(&self) -> Result<Weapon, serde_json::Error> {
        let item_result: Result<Weapon, serde_json::Error> =
            serde_json::from_str(self.data.as_str());
        match item_result {
            Ok(mut item) => {
                item.book_name = self.book_name.clone();
                item.book_short_name = self.book_short_name.clone();

                item.id = self.id;
                item.active = self.active;
                item.created_by = self.created_by;
                item.deleted_by = self.deleted_by;
                item.updated_by = self.updated_by;

                item.created_on = self.created_on.clone();
                item.deleted_on = self.deleted_on.clone();
                item.updated_on = self.updated_on.clone();

                item.created_by_obj = self.created_by_user.clone();
                item.deleted_by_obj = self.deleted_by_user.clone();
                item.updated_by_obj = self.updated_by_user.clone();

                return Ok(item);
            }
            Err(err) => {
                println!("{}", self.data);
                return Err(err);
            }
        }
    }

    pub fn to_armor(&self) -> Result<Armor, serde_json::Error> {
        let item_result: Result<Armor, serde_json::Error> =
            serde_json::from_str(self.data.as_str());
        match item_result {
            Ok(mut item) => {
                item.book_name = self.book_name.clone();
                item.book_short_name = self.book_short_name.clone();

                item.id = self.id;
                item.active = self.active;
                item.created_by = self.created_by;
                item.deleted_by = self.deleted_by;
                item.updated_by = self.updated_by;

                item.created_on = self.created_on.clone();
                item.deleted_on = self.deleted_on.clone();
                item.updated_on = self.updated_on.clone();

                item.created_by_obj = self.created_by_user.clone();
                item.deleted_by_obj = self.deleted_by_user.clone();
                item.updated_by_obj = self.updated_by_user.clone();

                return Ok(item);
            }
            Err(err) => {
                println!("{}", self.data);
                return Err(err);
            }
        }
    }

    pub fn to_gear_enhancement(&self) -> Result<GearEnhancement, serde_json::Error> {
        let item_result: Result<GearEnhancement, serde_json::Error> =
            serde_json::from_str(self.data.as_str());
        match item_result {
            Ok(mut item) => {
                item.book_name = self.book_name.clone();
                item.book_short_name = self.book_short_name.clone();

                item.id = self.id;
                item.active = self.active;
                item.created_by = self.created_by;
                item.deleted_by = self.deleted_by;
                item.updated_by = self.updated_by;

                item.created_on = self.created_on.clone();
                item.deleted_on = self.deleted_on.clone();
                item.updated_on = self.updated_on.clone();

                item.created_by_obj = self.created_by_user.clone();
                item.deleted_by_obj = self.deleted_by_user.clone();
                item.updated_by_obj = self.updated_by_user.clone();

                return Ok(item);
            }
            Err(err) => {
                println!("{}", self.data);
                return Err(err);
            }
        }
    }
}
