use uuid::{Uuid};
use crate::player_character::PlayerCharacter;
use chrono::prelude::*;


#[derive(Debug, Clone)]
pub struct Race {
    pub name: String,

    pub uuid: Uuid,

    // #[serde(default)]
    pub summary: String,

    // #[serde(default)]
    pub book_id: u32,

    #[serde(default, alias="bookPage")]
    pub page: String,

    // #[serde(default)]
    #[serde(default)]
    pub created_on:  Option<DateTime<Utc>>,

    // #[serde(default)]
    #[serde(default)]
    pub updated_on:  Option<DateTime<Utc>>,

    // #[serde(default)]
    #[serde(default)]
    pub deleted_on:  Option<DateTime<Utc>>,

    #[serde(default)]
    pub created_by: u32,

    #[serde(default)]
    pub updated_by: u32,

    #[serde(default)]
    pub deleted_by: u32,

    #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub deleted: bool,

    #[serde(default)]
    pub active: bool,

    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub created_by_obj: Option<PublicUserInfo>,

    #[serde(default)]
    pub updated_by_obj: Option<PublicUserInfo>,

    #[serde(default)]
    pub deleted_by_obj: Option<PublicUserInfo>,

}

impl Race {

    pub fn new() -> Race {
        //use the . operator to fetch the value of a field via the self keyword
        Race{
            active: true,
            book_id: 0,
            name: "".to_owned(),
            uuid: Uuid::new_v4(),
            created_on: None,
            updated_on: None,
            deleted_on: None,

            created_by: 0,
            updated_by: 0,
            deleted_by: 0,

            deleted: false,

            book_id: 0,
            page: "",to_owned(),
            description: "",to_owned(),

            book_name: None,
            book_short_name: None,

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