use crate::player_character::PlayerCharacter;
use crate::public_user_info::PublicUserInfo;
use chrono::prelude::*;
use serde;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, PartialEq, Serialize, Clone, Debug)]
pub struct Edge {
    #[serde(default)]
    pub id: u32,

    // #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    #[serde(default)]
    pub is_custom: bool,

    // #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub custom_name: String,

    // #[serde(default)]
    pub summary: String,

    // #[serde(default)]
    pub book_id: u32,

    #[serde(default, alias = "bookPage")]
    pub page: String,

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
    pub no_select: bool,

    #[serde(default)]
    pub created_by: u32,

    #[serde(default)]
    pub updated_by: u32,

    #[serde(default)]
    pub deleted_by: u32,

    #[serde(default)]
    pub active: bool,

    #[serde(default, alias="noSelect")]
    pub cannot_be_selected: bool,

    #[serde(default)]
    pub created_by_obj: Option<PublicUserInfo>,

    #[serde(default)]
    pub updated_by_obj: Option<PublicUserInfo>,

    #[serde(default)]
    pub deleted_by_obj: Option<PublicUserInfo>,

    #[serde(default)]
    pub book_name: Option<String>,

    #[serde(default)]
    pub book_short_name: Option<String>,

    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub effects: Vec<String>,

    #[serde(default)]
    pub conflicts: Vec<String>,

    #[serde(default)]
    pub abilities: Vec<String>,
}

impl Default for Edge {
    fn default() -> Self {
        //use the . operator to fetch the value of a field via the self keyword
        Self {
            active: true,
            id: 0,
            book_id: 0,
            is_custom: false,
            name: "".to_owned(),
            summary: "".to_owned(),
            description: "".to_owned(),
            custom_name: "".to_owned(),
            uuid: Uuid::new_v4(),
            created_on: None,
            updated_on: None,
            deleted_on: None,
            deleted: false,
            page: "".to_owned(),
            created_by: 0,
            updated_by: 0,
            deleted_by: 0,
            abilities: Vec::new(),

            no_select: false,

            cannot_be_selected: false,

            created_by_obj: None,
            deleted_by_obj: None,
            updated_by_obj: None,

            book_name: None,
            book_short_name: None,

            effects: Vec::new(),
            conflicts: Vec::new(),
        }
    }
}

impl Edge {

    pub fn get_name(&self) -> String {
        if self.custom_name.is_empty() {
            self.name.to_owned()
        } else {
            self.custom_name.to_owned()
        }
    }
    pub fn get_summary(&self) -> String {
        self.summary.to_owned()
    }
    pub fn apply(mut _char_obj: &PlayerCharacter) {}
}

impl Edge {
    // pub fn import_from_id(
    //     &mut self,
    //     id: u32,
    //     available_data: &GameDataPackage,
    // ) {

    //     for edge in available_data.edges.iter() {
    //         if edge.id == id {
    //             self.import_from_definition( edge.id, &edge );
    //             return;
    //         }
    //     }
    // }

    // pub fn import_from_definition(
    //     &mut self,
    //     id: u32,
    //     def: &Edge,
    // ) {
    //     self = def.clone();
    // }

    pub fn import_vars(&mut self, vars_option: &Option<EdgeVars>) {
        match vars_option {
            Some(vars) => {
                self.uuid = Uuid::parse_str(&vars.uuid).unwrap();
                self.custom_name = vars.custom_name.clone();
            }
            None => {}
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EdgeVars {
    #[serde(default)]
    pub custom_name: String,
    #[serde(default)]
    pub uuid: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct EdgeCombo {
    pub id: u32,
    #[serde(default, alias = "edgeOptions")]
    pub options: Option<EdgeVars>,
    #[serde(default)]
    pub def: Option<Edge>,
}

impl Default for EdgeCombo {
    fn default() -> Self {
        EdgeCombo {
            id: 0,
            options: None,
            def: None,
        }
    }
}
