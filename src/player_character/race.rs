use uuid::{Uuid};
use crate::player_character::PlayerCharacter;
use chrono::prelude::*;

#[derive(Debug, Clone)]
pub struct Race {
    pub name: String,

    pub uuid: Uuid,

    created_on:  Option<DateTime<Utc>>,
    updated_on:  Option<DateTime<Utc>>,
    deleted_on:  Option<DateTime<Utc>>,
    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub deleted: bool,
}

impl Race {

    pub fn new() -> Race {
        //use the . operator to fetch the value of a field via the self keyword
        Race{
            name: "".to_owned(),
            uuid: Uuid::new_v4(),
            created_on: None,
            updated_on: None,
            deleted_on: None,
            deleted: false,
        }
    }

    pub fn apply( mut _char_obj: &PlayerCharacter ) {

    }
}

// WASM Bindgen Getters/Setters

impl Race {

    pub fn set_name( &mut self, new_name: String) {
         self.name = new_name;
    }

    pub fn name( &self ) -> String {
        self.name.clone()
    }

    pub fn set_uuid( &mut self, new_value: String) {
        // self.uuid = uuid!( new_value[..] );
        self.uuid = Uuid::parse_str( &new_value ).unwrap();
    }

    pub fn uuid( &self ) -> String {
        self.uuid.to_owned()
    }
}