// use wasm_bindgen::prelude::*;
// pub mod attributes;
// pub mod edge;
// pub mod hindrance;
// pub mod exports;
pub mod imports;

use crate::json_data;

use uuid::{Uuid};
// use attributes::Attributes;
// use edge::Edge;
// use hindrance::Hindrance;
// use chrono::prelude::*;
use json_data::json_chargen_data::JSONChargenData;
// use json_data::json_chargen_book::JSONBookDefinition;
// use json_data::json_chargen_edge::JSONEdgeDefinition;
// use json_data::json_chargen_hindrance::JSONHindranceDefinition;
// use json_data::json_chargen_weapon::JSONWeaponDefinition;
// use json_data::json_chargen_armor::JSONArmorDefinition;
// use json_data::json_chargen_gear::JSONGearDefinition;
// use json_data::json_chargen_setting::JSONSettingDefinition;
// // use serde::{Serialize, Deserialize};

// use std::collections::HashMap;

// use serde::{Serialize, Deserialize};

#[derive(Clone, Debug)]

pub struct Setting {

    pub name: String,

    pub uuid: Uuid,

    available_data: JSONChargenData,
}

// WASM Bindgen Getters/Setters

impl Setting {

    pub fn new(
        available_data: String,
    ) -> Setting {
        //use the . operator to fetch the value of a field via the self keyword
        let setting = Setting{
            name: "".to_owned(),
            uuid: Uuid::new_v4(),
            available_data: serde_json::from_str(&available_data).unwrap(),
        };

        setting
    }

    pub fn new_import(
        available_data: JSONChargenData,
    ) -> Setting {
        //use the . operator to fetch the value of a field via the self keyword
        let setting = Setting{
            name: "".to_owned(),
            uuid: Uuid::new_v4(),
            available_data: available_data.clone(),
        };

        setting
    }

 }

 impl Setting {

    pub fn set_uuid( &mut self, new_value: String) {
        // self.uuid = uuid!( new_value[..] );
        self.uuid = Uuid::parse_str( &new_value ).unwrap();
    }

    pub fn uuid( &self ) -> String {
        self.uuid.to_string()
    }
 }