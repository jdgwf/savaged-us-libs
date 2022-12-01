use serde::{Serialize, Deserialize};
use super::json_chargen_book::JSONBookDefinition;
use super::json_chargen_edge::JSONEdgeDefinition;
use super::json_chargen_hindrance::JSONHindranceDefinition;

use super::json_chargen_armor::JSONArmorDefinition;
use super::json_chargen_gear::JSONGearDefinition;
use super::json_chargen_weapon::JSONWeaponDefinition;
use super::json_chargen_setting::JSONSettingDefinition;

#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct JSONChargenData {
    pub books: Vec<JSONBookDefinition>,

    pub edges: Vec<JSONEdgeDefinition>,
    pub hindrances: Vec<JSONHindranceDefinition>,

    pub gear: Vec<JSONGearDefinition>,
    pub armor: Vec<JSONArmorDefinition>,
    pub weapons: Vec<JSONWeaponDefinition>,
    pub settings: Vec<JSONSettingDefinition>,
}

