use crate::player_character::PlayerCharacter;
use crate::public_user_info::PublicUserInfo;
use chrono::prelude::*;
use serde;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, PartialEq, Serialize, Clone, Debug)]
pub struct GearEnhancement {
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
    pub no_select: bool,

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
    pub name_prefix: String,
    #[serde(default)]
    pub name_suffix: String,

    #[serde(default)]
    pub ammunition_cost: f32,
    #[serde(default)]
    pub ammunition_ap: f32,
    #[serde(default)]
    pub ammunition_weight_multiplier: f32,

    #[serde(default)]
    pub for_armor: bool,
    #[serde(default)]
    pub for_shield: bool,
    #[serde(default)]
    pub for_weapon: bool,
    #[serde(default)]
    pub for_ammo: bool,


    #[serde(default)]
    pub weapon_min_str_adjustment: f32,
    #[serde(default)]
    pub weapon_weight_multiplier: f32,

    #[serde(default)]
    pub weapon_cost_adjustment: f32,
    #[serde(default)]
    pub weapon_cost_adjustment_is_multiplied: bool,
    #[serde(default)]
    pub weapon_cost_adjustment_is_per_pound: bool,
    #[serde(default)]
    pub weapon_cost_adjustment_is_per_ap: bool,

    #[serde(default)]
    pub weapon_accuracy: f32,
    #[serde(default)]
    pub weapon_parry: f32,
    #[serde(default)]
    pub weapon_damage: f32,
    #[serde(default)]
    pub weapon_ap: f32,

    #[serde(default)]
    armor_min_str_adjustment: f32,
    #[serde(default)]
    pub armor_weight_multiplier: f32,

    #[serde(default)]
    pub armor_cost_adjustment: f32,
    #[serde(default)]
    pub armor_hardness_bonus: f32,
    #[serde(default)]
    pub armor_cost_adjustment_is_multiplied: bool,
    #[serde(default)]
    pub armor_cost_adjustment_is_per_pound: bool,
    #[serde(default)]
    pub armor_cost_adjustment_is_per_armor_value: bool,

    #[serde(default)]
    pub armor_armor_bonus: f32,

    #[serde(default)]
    pub shield_min_str_adjustment: f32,
    #[serde(default)]
    pub shield_weight_multiplier: f32,

    #[serde(default)]
    pub shield_cost_adjustment: f32,
    #[serde(default)]
    pub shield_hardness_bonus: f32,
    #[serde(default)]
    pub shield_cost_adjustment_is_multiplied: bool,
    #[serde(default)]
    pub shield_cost_adjustment_is_per_pound: bool,

    #[serde(default)]
    pub shield_parry_bonus: f32,

}

impl GearEnhancement {

    pub fn get_name(&self) -> String {
        if self.custom_name.is_empty() {
            self.name.clone()
        } else {
            self.custom_name.clone()
        }
    }
    pub fn get_summary(&self) -> String {
        "get,summary".to_string()
    }

    pub fn apply(mut _char_obj: &PlayerCharacter) {}
}

impl GearEnhancement {
    // pub fn import_from_id(
    //     &mut self,
    //     id: u32,
    //     available_data: &GameDataPackage,
    // ) {

    //     for gear_enhancement in available_data.gear_enhancements.iter() {
    //         if gear_enhancement.id == id {
    //             self.import_from_definition( gear_enhancement.id, &gear_enhancement );
    //             return;
    //         }
    //     }
    // }

    // pub fn import_from_definition(
    //     &mut self,
    //     id: u32,
    //     def: &GearEnhancement,
    // ) {
    //     self = def.clone();
    // }

    pub fn import_vars(&mut self, vars_option: &Option<GearEnhancementVars>) {
        match vars_option {
            Some(vars) => {
                self.uuid = Uuid::parse_str(&vars.uuid).unwrap();
                self.custom_name = vars.custom_name.clone();
            }
            None => {}
        }
    }
}

impl Default for GearEnhancement {
    fn default() -> Self {

        GearEnhancement {
            active: true,
            no_select: false,
            id: 0,
            book_id: 0,
            is_custom: false,
            name: "".to_owned(),
            summary: "".to_owned(),
            description: "".to_owned(),
            page: "".to_owned(),
            custom_name: "".to_owned(),
            uuid: Uuid::new_v4(),
            created_on: None,
            updated_on: None,
            deleted_on: None,
            deleted: false,

            created_by: 0,
            updated_by: 0,
            deleted_by: 0,

            created_by_obj: None,
            deleted_by_obj: None,
            updated_by_obj: None,

            book_name: None,
            book_short_name: None,


            name_prefix: "".to_owned(),
            name_suffix: "".to_owned(),

            ammunition_cost: 0.0,
            ammunition_ap: 0.0,
            ammunition_weight_multiplier: 1.0,

            for_armor: false,
            for_shield: false,
            for_weapon: false,
            for_ammo: false,


            weapon_min_str_adjustment: 0.0,
            weapon_weight_multiplier: 1.0,

            weapon_cost_adjustment: 0.0,
            weapon_cost_adjustment_is_multiplied: false,
            weapon_cost_adjustment_is_per_pound: false,
            weapon_cost_adjustment_is_per_ap: false,

            weapon_accuracy: 0.0,
            weapon_damage: 0.0,
            weapon_parry: 0.0,
            weapon_ap: 0.0,

            armor_min_str_adjustment: 0.0,
            armor_weight_multiplier: 1.0,

            armor_cost_adjustment: 0.0,
            armor_hardness_bonus: 0.0,
            armor_cost_adjustment_is_multiplied: false,
            armor_cost_adjustment_is_per_pound: false,
            armor_cost_adjustment_is_per_armor_value: false,

            armor_armor_bonus: 0.0,

            shield_min_str_adjustment: 0.0,
            shield_weight_multiplier: 1.0,

            shield_cost_adjustment: 0.0,
            shield_hardness_bonus: 0.0,
            shield_cost_adjustment_is_multiplied: false,
            shield_cost_adjustment_is_per_pound: false,

            shield_parry_bonus: 0.0,

        }

    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GearEnhancementVars {
    #[serde(default)]
    pub custom_name: String,
    #[serde(default)]
    pub uuid: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct GearEnhancementCombo {
    pub id: u32,
    #[serde(default, alias = "gear_enhancementOptions")]
    pub options: Option<GearEnhancementVars>,
    #[serde(default)]
    pub def: Option<GearEnhancement>,
}

impl Default for GearEnhancementCombo {
    fn default() -> Self {
        GearEnhancementCombo {
            id: 0,
            options: None,
            def: None,
        }
    }
}

#[derive(Deserialize, PartialEq, Serialize, Clone, Debug)]
pub struct GearEnhancementProfile {
    #[serde(default)]
    pub name:  String,
    #[serde(default)]
    pub damage:  String,
    #[serde(default, alias = "damageWithBrackets")]
    pub damage_with_brackets:  String,
    #[serde(default)]
    pub damage_original:  String,
    #[serde(default)]
    pub parry_modifier: i32,
    #[serde(default)]
    pub range:  String,
    #[serde(default)]
    pub reach:  u32,
    #[serde(default)]
    pub requires_2_hands: bool,
    #[serde(default)]
    pub rof:  u32,
    #[serde(default)]
    pub shots:  u32,
    #[serde(default, alias = "currentShots")]
    pub current_shots:  u32,
    #[serde(default)]
    pub heavy_gear_enhancement: bool,
    #[serde(default)]
    pub melee_only: bool,
    #[serde(default)]
    pub counts_as_innate: bool,
    #[serde(default)]
    pub notes:  String,
    #[serde(default)]
    pub equipped: bool,

    #[serde(default, alias = "toHitMod")]
    pub to_hit_mod:  i32,

    #[serde(default, alias = "damageDiceBase")]
    pub damage_dice_base:  String,
    #[serde(default, alias = "damageDiceBasePlus")]
    pub damage_dice_base_plus:  i32,

    #[serde(default)]
    pub is_shield: bool,
    #[serde(default)]
    pub thrown_gear_enhancement: bool,

    #[serde(default)]
    pub usable_in_melee: bool,
    #[serde(default)]
    pub add_strength_to_damage: bool,
    #[serde(default)]
    pub ap:  i32,
    #[serde(default)]
    pub ap_vs_rigid_armor_only:  u32,

    #[serde(default)]
    pub vtt_only: bool,

    #[serde(default, alias = "skillName")]
    pub skill_name:  String,
    #[serde(default, alias = "skillValue")]
    pub skill_value:  String,
}

impl Default for GearEnhancementProfile {
    fn default() -> Self {
        GearEnhancementProfile {
            name:  "".to_string(),
            damage:  "".to_string(),
            damage_with_brackets:  "".to_string(),
            damage_original:  "".to_string(),
            parry_modifier:  0,
            range:  "".to_string(),
            reach:  0,
            requires_2_hands: false,
            rof:  0,
            shots:  0,
            current_shots:  0,

            heavy_gear_enhancement: false,
            melee_only: false,
            counts_as_innate: false,
            notes:  "".to_string(),
            equipped: false,

            to_hit_mod:  0,

            damage_dice_base:  "".to_string(),
            damage_dice_base_plus:  0,

            is_shield: false,
            thrown_gear_enhancement: false,

            usable_in_melee: false,
            add_strength_to_damage: false,
            ap:  0,
            ap_vs_rigid_armor_only:  0,

            vtt_only: false,

            skill_name:  "".to_string(),
            skill_value:  "".to_string(),
        }
    }
}