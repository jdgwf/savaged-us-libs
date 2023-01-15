pub mod attributes;
pub mod edge;
pub mod hindrance;
pub mod exports;
pub mod gear;
pub mod armor;
pub mod weapon;
pub mod imports;
pub mod game_data_package;
pub mod character_export;

use uuid::{Uuid};
use attributes::Attributes;
use edge::Edge;
use crate::{setting::Setting, book::Book};
use hindrance::Hindrance;
use chrono::prelude::*;
use self::game_data_package::GameDataPackage;
use serde::{Serialize, Deserialize};
use serde;

#[derive(Deserialize,Serialize, Clone, Debug)]
pub struct PlayerCharacter {

    pub name: String,

    #[serde(default)]
    pub uuid: Uuid,

    pub attributes: Attributes,

    pub selected_edges: Vec< Edge >,
    pub selected_hindrances: Vec< Hindrance >,

    pub added_edges: Vec< Edge >,

    pub added_hindrances: Vec< Hindrance >,

    pub created_on:  Option<DateTime<Utc>>,
    pub updated_on:  Option<DateTime<Utc>>,
    pub deleted_on:  Option<DateTime<Utc>>,
    pub deleted: bool,

    pub setting: Setting,

    available_data: GameDataPackage,
}

impl PlayerCharacter {

    pub fn set_attribute_selected_agility( &mut self, new_val: u8 ) {
        self.attributes.selected_agility = new_val;
    }
    pub fn set_attribute_selected_smarts( &mut self, new_val: u8 ) {
        self.attributes.selected_smarts = new_val;
    }
    pub fn set_attribute_selected_spirit( &mut self, new_val: u8 ) {
        self.attributes.selected_spirit = new_val;
    }
    pub fn set_attribute_selected_strength( &mut self, new_val: u8 ) {
        self.attributes.selected_strength = new_val;
    }
    pub fn set_attribute_selected_vigor( &mut self, new_val: u8 ) {
        self.attributes.selected_vigor = new_val;
    }

    pub fn calc( &mut self ) {
        self._calc_reset();
    }

    fn _calc_reset( &mut self ) {
        self.attributes.reset();

        self.added_edges = Vec::new();
        self.added_hindrances = Vec::new();
    }
}

// WASM Bindgen Getters/Setters

impl PlayerCharacter {

    pub fn new(
        available_data: GameDataPackage,
    ) -> PlayerCharacter {
        //use the . operator to fetch the value of a field via the self keyword
        let mut pc = PlayerCharacter{
            name: "".to_owned(),
            uuid: Uuid::new_v4(),
            attributes: Attributes::new(),
            selected_edges: Vec::new(),
            selected_hindrances: Vec::new(),
            added_edges: Vec::new(),
            added_hindrances: Vec::new(),
            created_on: Some(Utc::now()),
            updated_on: Some(Utc::now()),
            deleted_on: Some(Utc::now()),
            deleted: false,
            available_data: available_data.clone(),
            setting: Setting::new(available_data.clone()),
        };

        pc.calc();
        pc
    }

    //
    // pub fn set_name( &mut self, new_name: String) {
    //      self.name = new_name;
    // }

    //
    // pub fn name( &self ) -> String {
    //     self.name.clone()
    // }

    pub fn get_available_edges_count( &self ) -> usize {
        self.available_data.edges.len()
    }

    pub fn get_available_books_count( &self ) -> usize {
        self.available_data.books.len()
    }

    pub fn get_available_books_json( &self ) -> String {
        serde_json::to_string( &self.available_data.books ).unwrap()
    }

    pub fn get_available_hindrances_json( &self ) -> String {
        serde_json::to_string( &self.available_data.hindrances ).unwrap()
    }
    pub fn get_available_edges_json( &self ) -> String {
        serde_json::to_string( &self.available_data.edges ).unwrap()
    }

    // pub fn get_available_weapons_json( &self ) -> String {
    //     serde_json::to_string( &self.available_data.weapons ).unwrap()
    // }
    // pub fn get_available_armor_json( &self ) -> String {
    //     serde_json::to_string( &self.available_data.armor ).unwrap()
    // }
    // pub fn get_available_gear_json( &self ) -> String {
    //     serde_json::to_string( &self.available_data.gear ).unwrap()
    // }

    // pub fn get_available_settings_json( &self ) -> String {
    //     serde_json::to_string( &self.available_data.settings ).unwrap()
    // }

    // pub fn get_available_hindrances_count( &self ) -> usize {
    //     self.available_data.hindrances.len()
    // }

    // pub fn get_available_weapon_count( &self ) -> usize {
    //     self.available_data.weapons.len()
    // }

    // pub fn get_available_armor_count( &self ) -> usize {
    //     self.available_data.armor.len()
    // }

    // pub fn get_available_gear_count( &self ) -> usize {
    //     self.available_data.gear.len()
    // }
    //
    // pub fn created_on( &self ) -> DateTime<Utc> {
    //     self.created_on.clone()
    // }

    //
    // pub fn updated_on( &self ) -> DateTime<Utc> {
    //     self.updated_on.clone()
    // }

    //
    // pub fn deleted_on( &self ) -> DateTime<Utc> {
    //     self.deleted_on.clone()
    // }

    // pub fn set_uuid( &mut self, new_value: String) {
    //     // self.uuid = uuid!( new_value[..] );
    //     self.uuid = Uuid::parse_str( &new_value ).unwrap();
    // }

    // pub fn uuid( &self ) -> String {
    //     self.uuid.to_string()
    // }

    // pub fn attributes( &self ) -> Attributes {
    //     self.attributes.clone()
    // }

    // pub fn set_attributes( &mut self, new_value: Attributes) {
    //     // self.uuid = uuid!( new_value[..] );
    //     self.attributes = new_value.clone();
    // }

    pub fn reset( &mut self ) {
        self.name = "".to_owned();
        self.uuid = Uuid::new_v4();
        self.attributes = Attributes::new();
        self.selected_edges = Vec::new();
        self.selected_hindrances = Vec::new();
        self.added_edges = Vec::new();
        self.added_hindrances = Vec::new();
    }

}

// non WASM functions
impl PlayerCharacter {

    pub fn get_available_books( &self ) -> &Vec< Book > {
        &self.available_data.books
    }

    pub fn get_available_hindrances( &self ) -> &Vec< Hindrance > {
        &self.available_data.hindrances
    }

    pub fn get_available_edges( &self ) -> &Vec< Edge > {
        &self.available_data.edges
    }

    // pub fn get_available_weapons( &self ) -> &Vec< JSONWeaponDefinition > {
    //     &self.available_data.weapons
    // }

    // pub fn get_available_armor( &self ) -> &Vec< JSONArmorDefinition > {
    //     &self.available_data.armor
    // }

    // pub fn get_available_gear( &self ) -> &Vec< JSONGearDefinition > {
    //     &self.available_data.gear
    // }

    // pub fn get_available_settings( &self ) -> &Vec< JSONSettingDefinition > {
    //     &self.available_data.settings
    // }
}

// setting data functions

impl PlayerCharacter {
    pub fn setting_set_name( &mut self, new_name: String) {
        self.setting.name = new_name;
    }

    pub fn setting_get_name( &mut self ) -> String {
        self.setting.name.clone()
    }
}