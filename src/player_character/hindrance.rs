use uuid::{Uuid};
use crate::{player_character::PlayerCharacter, json_data::{json_chargen_hindrance::{JSONHindranceDefinition, JSONHindranceVars}, json_chargen_data::JSONChargenData}};
use chrono::prelude::*;
use super::super::utils::bool_from_int_or_bool;
// use super::super::utils::string_to_uuid;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Clone, Debug)]
pub struct Hindrance {
    #[serde(default)]
    pub name: String,

    pub id: u32,
    // #[serde(deserialize_with = "string_to_uuid")]
    pub uuid: Uuid,

    #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub is_custom: bool,

    #[serde(default)]
    pub custom_name: String,
    #[serde(default)]
    pub created_on:  Option<DateTime<Utc>>,
    #[serde(default)]
    pub updated_on:  Option<DateTime<Utc>>,
    #[serde(default)]
    pub deleted_on:  Option<DateTime<Utc>>,
    #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub deleted: bool,
}

impl Hindrance {

    pub fn new() -> Hindrance {
        //use the . operator to fetch the value of a field via the self keyword
        Hindrance{
            name: "".to_owned(),
            uuid: Uuid::new_v4(),
            created_on: None,
            id: 0,
            updated_on: None,
            custom_name: "".to_owned(),
            is_custom: false,
            deleted_on: None,
            deleted: false,
        }
    }
    pub fn apply( mut char_obj: &PlayerCharacter ) {

    }
}

impl Hindrance {
    pub fn import_from_id(
        &mut self,
        id: u32,
        def: JSONHindranceDefinition,
        available_data: &JSONChargenData,
    ) {

        for hindrance in available_data.hindrances.iter() {
            if hindrance.id == id {
                self.import_from_definition( hindrance.id, &hindrance );
                return
            }
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

    pub fn import_from_definition(
        &mut self,
        id: u32,
        def: &JSONHindranceDefinition,
    ) {
        self.name = def.name.clone();
        if id == 0 {
            self.is_custom = true;
        } else {
            self.is_custom = false;
        }
        self.id = id;

        self.created_on = Some(DateTime::from_utc(DateTime::parse_from_rfc3339( &def.created_on ).unwrap().naive_utc(), Utc));
        self.updated_on = Some(DateTime::from_utc(DateTime::parse_from_rfc3339( &def.updated_on ).unwrap().naive_utc(), Utc));
        self.deleted_on = Some(DateTime::from_utc(DateTime::parse_from_rfc3339( &def.deleted_on ).unwrap().naive_utc(), Utc));

    }

    pub fn import_vars(
        &mut self,
        vars: &JSONHindranceVars,
    ) {
        self.uuid = Uuid::parse_str( &vars.uuid ).unwrap();
        self.custom_name = vars.custom_name.clone();
    }

}