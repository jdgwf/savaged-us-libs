use uuid::{Uuid};
use crate::player_character::PlayerCharacter;
use crate::utils::array_to_string;
// use std::collections::HashMap;
// use crate::json_data::json_chargen_data::ChargenData;
// use crate::json_data::json_chargen_weapon::JSONWeaponDefinition;
// use crate::json_data::json_chargen_weapon::JSONWeaponVars;
use chrono::prelude::*;
// use super::super::utils::bool_from_int_or_bool;
// use super::chargen_data::ChargenData;
// use super::super::utils::string_to_uuid;
use serde::{Serialize, Deserialize};
use serde;

#[derive(Deserialize,Serialize, Clone, Debug, PartialEq)]
pub struct Weapon {

    #[serde(default)]
    pub id: u32,

    // #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    #[serde(default)]
    pub is_custom: bool,

    // #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub custom_name: String,

    // #[serde(default, deserialize_with = "array_to_string")]
    // pub description: String,

    // #[serde(default)]
    pub summary: String,

    // #[serde(default)]
    pub book_id: u32,

    // #[serde(deserialize_with = "string_to_uuid")]
    #[serde(default)]
    pub uuid: Uuid,

    // #[serde(default)]
    pub created_on: Option<DateTime<Utc>>,

    // #[serde(default)]
    pub updated_on: Option<DateTime<Utc>>,

    // #[serde(default)]
    pub deleted_on: Option<DateTime<Utc>>,

    // #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    #[serde(default)]
    pub deleted: bool,

    #[serde(default)]
    pub created_by: u32,

    #[serde(default)]
    pub updated_by: u32,

    #[serde(default)]
    pub deleted_by: u32,
}

impl Weapon {

    pub fn new() -> Weapon {
        //use the . operator to fetch the value of a field via the self keyword
        Weapon{
            id: 0,
            book_id: 0,
            is_custom: false,
            name: "".to_owned(),
            summary: "".to_owned(),
            // description: "".to_owned(),
            custom_name: "".to_owned(),
            uuid: Uuid::new_v4(),
            created_on: None,
            updated_on: None,
            deleted_on: None,
            deleted: false,

            created_by: 0,
            updated_by: 0,
            deleted_by: 0,
        }
    }

    pub fn get_name(
        &self
    ) -> String {
        if self.custom_name.is_empty() {
            self.name.clone()
        } else {
            self.custom_name.clone()
        }
    }

    pub fn apply( mut char_obj: &PlayerCharacter ) {

    }
}

impl Weapon {
    // pub fn import_from_id(
    //     &mut self,
    //     id: u32,
    //     available_data: &ChargenData,
    // ) {

    //     for weapon in available_data.weapons.iter() {
    //         if weapon.id == id {
    //             self.import_from_definition( weapon.id, &weapon );
    //             return;
    //         }
    //     }
    // }

    // pub fn import_from_definition(
    //     &mut self,
    //     id: u32,
    //     def: &Weapon,
    // ) {
    //     self = def.clone();
    // }

    pub fn import_vars(
        &mut self,
        vars_option: &Option<WeaponVars>,
    ) {
        match vars_option {
            Some( vars ) => {
                self.uuid = Uuid::parse_str( &vars.uuid ).unwrap();
                self.custom_name = vars.custom_name.clone();
            }
            None => {}
        }

    }

}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WeaponVars {
    #[serde(default)]
    pub custom_name: String,
    #[serde(default)]
    pub uuid: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WeaponCombo {
    pub id: u32,
    #[serde(default, alias = "weaponOptions")]
    pub options: Option<WeaponVars>,
    #[serde(default)]
    pub def: Option<Weapon>,
}

impl Default for WeaponCombo {
    fn default() -> Self {
        WeaponCombo {
            id: 0,
            options: None,
            def: None,
        }
    }
}
