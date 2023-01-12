use uuid::{Uuid};
use crate::{player_character::PlayerCharacter, utils::array_to_string};
use chrono::prelude::*;
// use super::super::utils::bool_from_int_or_bool;
// use super::super::utils::string_to_uuid;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct Hindrance {
    // #[serde(default)]
    pub name: String,

    // #[serde(default)]
    pub summary: String,

    #[serde(default, deserialize_with = "array_to_string")]
    pub description: String,

    #[serde(default)]
    pub id: u32,

    pub book_id: u32,

    // #[serde(deserialize_with = "string_to_uuid")]
    #[serde(default)]
    pub uuid: Uuid,

    // #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    #[serde(default)]
    pub is_custom: bool,

    // #[serde(default)]
    pub effects: Vec<String>,

    #[serde(default)]
    pub custom_name: String,

    // #[serde(default)]
    pub created_on:  Option<DateTime<Utc>>,

    // #[serde(default)]
    pub updated_on:  Option<DateTime<Utc>>,

    // #[serde(default)]
    pub deleted_on:  Option<DateTime<Utc>>,

    // #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    #[serde(default)]
    pub deleted: bool,


    #[serde(default)]
    pub created_by: u32,

    #[serde(default)]
    pub updated_by: u32,

    #[serde(default)]
    pub deleted_by: u32,



    pub base_name: String,
    pub no_select: bool,
    pub hidden_on_character_sheet: bool,

    // custom_name: String,
    // uuid: String,

    pub setting_item: bool,
    pub counts_as_other: Vec<String>,
    pub major: bool,
    pub minor_or_major: bool,
    pub summary_minor: String,
    pub effects_minor: Vec<String>,

    pub conflicts: Vec<String>,
    pub needs_to_specify: bool,
    pub always_show_long_name: bool,
    pub can_be_taken_more_than_once: bool,

    pub removed: bool,
    // description: Vec<String>,

    pub specify: Option<String>,
}

impl Hindrance {

    pub fn new() -> Hindrance {
        //use the . operator to fetch the value of a field via the self keyword
        Hindrance{
            name: "".to_owned(),
            summary: "".to_owned(),
            description: "".to_owned(),
            uuid: Uuid::new_v4(),
            created_on: None,
            id: 0,
            book_id: 0,
            effects: Vec::new(),
            updated_on: None,
            custom_name: "".to_owned(),
            is_custom: false,
            deleted_on: None,
            deleted: false,

            created_by: 0,
            updated_by: 0,
            deleted_by: 0,

            base_name: "".to_owned(),
            no_select: false,
            hidden_on_character_sheet: false,

            setting_item: false,
            counts_as_other: Vec::new(),
            major: false,
            minor_or_major: false,
            summary_minor: "".to_owned(),
            effects_minor: Vec::new(),

            conflicts: Vec::new(),
            needs_to_specify: false,
            always_show_long_name: false,
            can_be_taken_more_than_once: false,

            removed: false,
            specify: None,
        }
    }
    pub fn apply( mut char_obj: &PlayerCharacter ) {

    }
}

impl Hindrance {
    // pub fn import_from_id(
    //     &mut self,
    //     id: u32,
    //     def: JSONHindranceDefinition,
    //     available_data: &ChargenData,
    // ) {

    //     for hindrance in available_data.hindrances.iter() {
    //         if hindrance.id == id {
    //             self.import_from_definition( hindrance.id, &hindrance );
    //             return
    //         }
    //     }

    // }

    pub fn get_name(
        &self
    ) -> String {
        if self.custom_name.is_empty() {
            self.name.clone()
        } else {
            self.custom_name.clone()
        }
    }

    // pub fn import_from_definition(
    //     &mut self,
    //     id: u32,
    //     def: &Hindrance,
    // ) {
    //     self.name = def.name.clone();
    //     if id == 0 {
    //         self.is_custom = true;
    //     } else {
    //         self.is_custom = false;
    //     }
    //     self.id = id;

    //     self.created_on = Some(DateTime::from_utc(DateTime::parse_from_rfc3339( &def.created_on ).unwrap().naive_utc(), Utc));
    //     self.updated_on = Some(DateTime::from_utc(DateTime::parse_from_rfc3339( &def.updated_on ).unwrap().naive_utc(), Utc));
    //     self.deleted_on = Some(DateTime::from_utc(DateTime::parse_from_rfc3339( &def.deleted_on ).unwrap().naive_utc(), Utc));

    // }

    pub fn import_vars(
        &mut self,
        vars_option: &Option<HindranceVars>,
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
pub struct HindranceVars {
    #[serde(default)]
    pub custom_name: String,
    #[serde(default)]
    pub uuid: String,
}



#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct HindranceCombo {
    pub id: u32,
    #[serde(default, alias = "hindOptions")]
    pub options: Option<HindranceVars>,
    #[serde(default)]
    pub def: Option<Hindrance>,
}


impl Default for HindranceCombo {
    fn default() -> Self {
        HindranceCombo {
            id: 0,
            options: None,
            def: None,
        }
    }
}
