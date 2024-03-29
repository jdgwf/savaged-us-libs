use crate::player_character::PlayerCharacter;
use crate::public_user_info::PublicUserInfo;
use crate::player_character::weapon::WeaponProfile;
// use crate::utils::bool_from_int_or_bool;
use chrono::prelude::*;
// use serde_repr::*;
use serde_repr::{Serialize_repr, Deserialize_repr};
use serde;
// use serde_with::serde_as;
// use serde_with::DefaultOnError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
// use super::weapon::{Weapon, WeaponProfile};

// #[serde_as]
#[derive(Deserialize, PartialEq, Serialize, Clone, Debug )]
pub struct Armor {
    #[serde(default)]
    pub id: u32,

    // #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    #[serde(default)]
    pub is_custom: bool,

    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub custom_name: String,

    #[serde(default)]
    pub description: String,

    #[serde(default)]
    pub summary: String,

    #[serde(default)]
    pub book_id: u32,

    #[serde(default)]
    pub no_select: bool,

    // #[serde(default, alias = "bookPage")]
    #[serde(default)]
    pub page: String,

    // #[serde(deserialize_with = "string_to_uuid")]
    #[serde(default)]
    pub uuid: Uuid,

    // #[serde(default)]
    #[serde(default)]
    pub created_on: Option<DateTime<Utc>>,

    // #[serde(default)]
    #[serde(default)]
    pub updated_on: Option<DateTime<Utc>>,

    // #[serde(default)]
    #[serde(default)]
    pub deleted_on: Option<DateTime<Utc>>,

    // #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    #[serde(default)]
    pub deleted: bool,

    #[serde(default)]
    pub created_by: u32,

    #[serde(default)]
    pub updated_by: u32,

    #[serde(default)]
    pub deleted_by: u32,

    // #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    #[serde(default)]
    pub active: bool,

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
    pub effects: Vec<String>,

    #[serde(default)]
    pub armor_value: u32,
    #[serde(default)]
    pub abilities: Vec<String>,
    #[serde(default)]
    pub hardness: u32,

    // #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    #[serde(default)]
    pub stacks_with_other_armor: bool,

    // #[serde_as(deserialize_as = "DefaultOnError")]
    #[serde(default)]
    pub secondary_armor_value: u32,

    #[serde(default)]
    pub toughness: u32,

    // #[serde_as(deserialize_as = "DefaultOnError")]
    #[serde(default)]
    pub pf_armor_type: u32,
    // pub pf_armor_type: PathfinderArmorType,

    #[serde(default)]
    pub ap_vs_lasers: u32,
    // #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    #[serde(default)]
    pub rigid_armor: bool,
    #[serde(default)]
    pub covers_head: bool,
    #[serde(default)]
    pub covers_face: bool,
    #[serde(default)]
    pub covers_torso: bool,
    #[serde(default)]
    pub covers_arms: bool,
    #[serde(default)]
    pub covers_legs: bool,
    #[serde(default)]
    pub negate_4_ap: bool,
    #[serde(default)]
    pub is_shield: bool,
    #[serde(default)]
    pub is_energy_screen: bool,
    #[serde(default)]
    pub shield_parry_bonus: u32,
    #[serde(default)]
    pub shield_armor_vs_ranged: i32,
    #[serde(default)]
    pub shield_cover_vs_ranged: i32,
    #[serde(default)]
    pub minimum_strength: String,

    // #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    #[serde(default)]
    pub requires_2_hands: bool,

    #[serde(default)]
    pub pace: u32,
    #[serde(default)]
    pub run: String,
    #[serde(default)]
    pub set_strength: String,

    // #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    #[serde(default)]
    pub heavy: bool,

    #[serde(default)]
    pub size: u32,
    #[serde(default)]
    pub default_model_label: String,

    #[serde(default)]
    pub alternate_modes: Vec<ArmorAlternateMode>,

    #[serde(default)]
    pub integrated_weapons: Vec<WeaponProfile>,

    #[serde(default)]
    pub zero_weight_when_equipped: bool,

    #[serde(default)]
    pub cost: f32,
    #[serde(default)]
    pub weight: f32,
    #[serde(default)]
    pub quantity: u32,

}

impl Armor {

    pub fn get_name(&self) -> String {
        if self.custom_name.is_empty() {
            self.name.to_owned()
        } else {
            self.custom_name.to_owned()
        }
    }

    pub fn get_summary(&self) -> String {
        self.summary.trim().to_owned()
    }

    pub fn apply(mut _char_obj: &PlayerCharacter) {}

    pub fn get_armor_value_hr(&self) -> String {

        if self.armor_value > 0 {
            return "Armor ".to_owned() + &self.armor_value.to_string();
        }

        "".to_owned()
    }

    pub fn basic_info(&self) -> String {
        let mut rv = "".to_owned();

        if self.is_shield {
            rv += &"Shield";
        } else {
            rv += &self.get_armor_value_hr();
        }

        if !self.get_summary().is_empty() {
            if !rv.is_empty() {
                rv += &"; ";
            }
            rv += &self.get_summary();
        }

        return rv;
    }

    pub fn import_vars(&mut self, vars_option: &Option<ArmorVars>) {
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
pub struct ArmorVars {
    #[serde(default)]
    pub custom_name: String,
    #[serde(default)]
    pub uuid: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ArmorCombo {
    pub id: u32,
    #[serde(default, alias = "armorOptions")]
    pub options: Option<ArmorVars>,
    #[serde(default)]
    pub def: Option<Armor>,
}

impl Default for ArmorCombo {
    fn default() -> Self {
        ArmorCombo {
            id: 0,
            options: None,
            def: None,
        }
    }
}

impl Default for Armor {
    fn default() -> Self {
        Armor {
            active: true,
            id: 0,
            no_select: false,
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

            created_by_obj: None,
            deleted_by_obj: None,
            updated_by_obj: None,

            book_name: None,
            book_short_name: None,

            effects: Vec::new(),

            armor_value: 0,
            abilities: Vec::new(),
            hardness: 0,
            stacks_with_other_armor: false,
            secondary_armor_value: 0,
            toughness: 0,
            // pf_armor_type: PathfinderArmorType::None,
            pf_armor_type: 0,
            ap_vs_lasers: 0,
            rigid_armor: false,
            covers_head: false,
            covers_face: false,
            covers_torso: false,
            covers_arms: false,
            covers_legs: false,
            negate_4_ap: false,
            is_shield: false,
            is_energy_screen: false,
            shield_parry_bonus: 0,
            shield_armor_vs_ranged: 0,
            shield_cover_vs_ranged: 0,
            minimum_strength: "".to_owned(),
            requires_2_hands: false,
            pace: 0,
            run: "".to_owned(),
            set_strength: "".to_owned(),
            heavy: false,
            size: 0,
            default_model_label: "".to_owned(),
            alternate_modes: Vec::new(),
            integrated_weapons: Vec::new(),
            zero_weight_when_equipped: false,

            cost: 0.0,
            weight: 0.0,
            quantity: 0,
        }
    }
}
#[derive(Deserialize, Serialize, PartialEq, Clone, Debug)]
pub struct ArmorAlternateMode {
    pub name: String,
    pub armor_value: u32,
    pub minimum_strength: String,
    pub secondary_armor_value: u32,
    pub toughness: u32,
    pub heavy: bool,
    pub effects: Vec<String>,
    pub weight: u32,
}

impl Default for ArmorAlternateMode {
    fn default() -> Self {
        ArmorAlternateMode {
            name: "".to_string(),
            armor_value: 0,
            minimum_strength: "".to_string(),
            secondary_armor_value: 0,
            toughness: 0,
            heavy: false,
            effects: Vec::new(),
            weight: 0,
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Clone, Debug)]
#[repr(u8)]
pub enum PathfinderArmorType {
    None = 0,
    Light = 1,
    Medium = 2,
    Heavy = 3,
}

impl Default for PathfinderArmorType {
    fn default() -> PathfinderArmorType {
        PathfinderArmorType::None
    }
}

