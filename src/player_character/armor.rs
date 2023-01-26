use crate::player_character::PlayerCharacter;
use crate::public_user_info::PublicUserInfo;
use chrono::prelude::*;
use serde_repr::*;
use serde;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::weapon::Weapon;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
#[serde(tag = "pf_armor_type")]
pub struct Armor {
    #[serde(default)]
    pub id: u32,

    // #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    #[serde(default)]
    pub is_custom: bool,

    // #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub custom_name: String,

    #[serde(default)]
    pub description: String,

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

    pub armor_value: u32,
    pub abilities: Vec<String>,
    pub hardness: u32,
    pub stacks_with_other_armor: bool,
    pub secondary_armor_value: String,
    pub toughness: u32,
    pub pf_armor_type: PathfinderArmorType,
    pub ap_vs_lasers: u32,
    pub rigid_armor: bool,
    pub covers_head: bool,
    pub covers_face: bool,
    pub covers_torso: bool,
    pub covers_arms: bool,
    pub covers_legs: bool,
    pub negate_4_ap: bool,
    pub is_shield: bool,
    pub is_energy_screen: bool,
    pub shield_parry_bonus: u32,
    pub shield_armor_vs_ranged: i32,
    pub shield_cover_vs_ranged: i32,
    pub minimum_strength: String,
    pub requires_2_hands: bool,
    pub pace: u32,
    pub run: String,
    pub set_strength: String,
    pub heavy: bool,
    pub size: u32,
    pub default_model_label: String,
    pub alternate_modes: Vec<ArmorAlternateMode>,
    pub integrated_weapons: Vec<Weapon>,
    pub zero_weight_when_equipped: bool,

    #[serde(default)]
    pub cost: f32,
    #[serde(default)]
    pub weight: f32,
    #[serde(default)]
    pub quantity: u32,
}

impl Armor {
    pub fn new() -> Armor {
        //use the . operator to fetch the value of a field via the self keyword
        Armor {
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
            secondary_armor_value: "".to_owned(),
            toughness: 0,

            pf_armor_type: PathfinderArmorType::None,
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

    pub fn get_name(&self) -> String {
        if self.custom_name.is_empty() {
            self.name.clone()
        } else {
            self.custom_name.clone()
        }
    }

    pub fn apply(mut _char_obj: &PlayerCharacter) {}
}

impl Armor {
    // pub fn import_from_id(
    //     &mut self,
    //     id: u32,
    //     available_data: &GameDataPackage,
    // ) {

    //     for armor in available_data.armors.iter() {
    //         if armor.id == id {
    //             self.import_from_definition( armor.id, &armor );
    //             return;
    //         }
    //     }
    // }

    // pub fn import_from_definition(
    //     &mut self,
    //     id: u32,
    //     def: &Armor,
    // ) {
    //     self = def.clone();
    // }

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
            secondary_armor_value: "".to_owned(),
            toughness: 0,
            pf_armor_type: PathfinderArmorType::None,
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
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ArmorAlternateMode {
    pub name: String,
    pub armor_value: u32,
    pub minimum_strength: String,
    pub secondary_armor_value: String,
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
            secondary_armor_value: "".to_string(),
            toughness: 0,
            heavy: false,
            effects: Vec::new(),
            weight: 0,
        }
    }
}

#[derive(Serialize_repr, Deserialize_repr, Clone, PartialEq, Debug)]
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