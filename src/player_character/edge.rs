use uuid::{Uuid};
use crate::player_character::PlayerCharacter;
// use std::collections::HashMap;
use crate::json_data::json_chargen_data::JSONChargenData;
use crate::json_data::json_chargen_edge::JSONEdgeDefinition;
use crate::json_data::json_chargen_edge::JSONEdgeVars;
use chrono::prelude::*;
use super::super::utils::bool_from_int_or_bool;
// use super::super::utils::string_to_uuid;
use serde::{Serialize, Deserialize};
use serde;

#[derive(Deserialize, Clone, Debug)]
pub struct Edge {

    pub id: u32,
    #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub is_custom: bool,

    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub custom_name: String,

    // #[serde(deserialize_with = "string_to_uuid")]
    #[serde(default)]
    pub uuid: Uuid,

    #[serde(default)]
    pub created_on: Option<DateTime<Utc>>,
    #[serde(default)]
    pub updated_on: Option<DateTime<Utc>>,
    #[serde(default)]
    pub deleted_on: Option<DateTime<Utc>>,
    #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub deleted: bool,

}

impl Edge {

    pub fn new() -> Edge {
        //use the . operator to fetch the value of a field via the self keyword
        Edge{
            id: 0,
            is_custom: false,
            name: "".to_owned(),
            custom_name: "".to_owned(),
            uuid: Uuid::new_v4(),
            created_on: None,
            updated_on: None,
            deleted_on: None,
            deleted: false,
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

impl Edge {
    pub fn import_from_id(
        &mut self,
        id: u32,
        available_data: &JSONChargenData,
    ) {

        for edge in available_data.edges.iter() {
            if edge.id == id {
                self.import_from_definition( edge.id, &edge );
                return;
            }
        }
    }

    pub fn import_from_definition(
        &mut self,
        id: u32,
        def: &JSONEdgeDefinition,
    ) {
        self.name = def.name.clone();
        if id == 0 {
            self.is_custom = true;
        } else {
            self.is_custom = false;
        }
        self.id = id;

        if !def.created_on.is_empty() {
            self.created_on = Some(DateTime::from_utc(DateTime::parse_from_rfc3339( &def.created_on ).unwrap().naive_utc(), Utc));
        }
        if !def.updated_on.is_empty() {
            self.updated_on = Some(DateTime::from_utc(DateTime::parse_from_rfc3339( &def.updated_on ).unwrap().naive_utc(), Utc));
        }
        if !def.deleted_on.is_empty() {
            self.deleted_on = Some(DateTime::from_utc(DateTime::parse_from_rfc3339( &def.deleted_on ).unwrap().naive_utc(), Utc));
        }
    }

    pub fn import_vars(
        &mut self,
        vars: &JSONEdgeVars,
    ) {
        self.uuid = Uuid::parse_str( &vars.uuid ).unwrap();
        self.custom_name = vars.custom_name.clone();
    }

}
