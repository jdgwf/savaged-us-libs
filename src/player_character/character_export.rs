use serde::{Serialize, Deserialize};
use crate::player_character::edge::EdgeCombo;
use crate::player_character::hindrance::HindranceCombo;
use crate::setting::Setting;

// use super::json_chargen_edge::EdgeCombo;
// use super::json_chargen_hindrance::HindranceDefinition;
// use super::json_chargen_setting::SettingDefinition;
use super::super::utils::bool_from_int_or_bool;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct BaseAttributes {
    pub agility: u8,
    pub smarts: u8,
    pub spirit: u8,
    pub strength: u8,
    pub vigor: u8,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RaceOptions {
    pub chosen_race_abilities: Vec<ChargenRaceAbility>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ChargenRaceAbility {
    pub adjusted_value: u32,
    pub custom_effects: Vec<String>,
    pub custom_name: String,
    pub custom_summary: String,
    pub custom_value: u32,
    pub effects: Vec<String>,
    pub max: String,
    #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub needs_selected_attribute: bool,
    #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub needs_selected_edge: bool,
    #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub needs_selected_hindrance: bool,
    #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub needs_selected_skill: bool,
    #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub needs_selected_trait: bool,
    #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub needs_selected_super_powers: bool,
    #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub needs_selected_power: bool,
    #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub positive: bool,
    pub selected_attribute: String,
    pub selected_edge: String,
    pub selected_hindrance: String,
    #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub selected_hindrance_major: bool,
    pub selected_skill: String,
    pub selected_skill_specify: String,
    pub selected_trait: String,
    pub selected_trait_specify: String,
    // pub selected_super_power_2021_options: SuperPower2021ObjectVars,
    pub selected_super_power_2021: u32,
    pub selected_super_power: u32,
    // pub selected_super_power_options: SuperPower2014ObjectVars,
    pub selected_power: u32,
    pub value: u32,

}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CharacterExport {
    #[serde(default)]
    pub id: u32,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub last_save_id: i64,
    // pub race_choices: RaceOptions,

    pub attribute_assignments: BaseAttributes,
    #[serde(default)]
    pub version: f64,
    #[serde(default)]
    pub session_id: u32,

    pub edges: Vec<EdgeCombo>,
    pub hindrances: Vec<HindranceCombo>,

    pub setting: Setting,

    #[serde(default)]
    pub created_on: String,
    #[serde(default)]
    pub updated_on: String,
    #[serde(default)]
    pub deleted_on: String,
    #[serde(default)]
    pub deleted: bool,
}