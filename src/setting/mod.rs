// use wasm_bindgen::prelude::*;
// pub mod attributes;
// pub mod edge;
// pub mod hindrance;
// pub mod exports;
pub mod imports;

use crate::{player_character::chargen_data::ChargenData};

use uuid::{Uuid};
// use attributes::Attributes;
// use edge::Edge;
// use hindrance::Hindrance;
// use chrono::prelude::*;

// // use serde::{Serialize, Deserialize};

// use std::collections::HashMap;

// use serde::{Serialize, Deserialize};

use serde::{Serialize, Deserialize};
use serde;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Setting {

    pub name: String,

    #[serde(default)]
    pub uuid: Uuid,

    // #[serde(skip)]
    // available_data: ChargenData,
}

// WASM Bindgen Getters/Setters

impl Setting {

    pub fn new(
        available_data: ChargenData,
    ) -> Setting {
        //use the . operator to fetch the value of a field via the self keyword
        let setting = Setting{
            name: "".to_owned(),
            uuid: Uuid::new_v4(),
            // available_data: available_data,
        };

        setting
    }

    pub fn new_import(
        available_data: ChargenData,
    ) -> Setting {
        //use the . operator to fetch the value of a field via the self keyword
        let setting = Setting{
            name: "".to_owned(),
            uuid: Uuid::new_v4(),
            // available_data: available_data.clone(),
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