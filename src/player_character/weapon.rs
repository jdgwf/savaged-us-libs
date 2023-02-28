use crate::player_character::PlayerCharacter;
use crate::public_user_info::PublicUserInfo;
use chrono::prelude::*;
use serde;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, PartialEq, Serialize, Clone, Debug)]
pub struct Weapon {
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
    pub no_select: bool,

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
    pub minimum_strength:  String,

    #[serde(default)]
    pub tw_cost:  f32,
    #[serde(default)]
    pub tw_effects:  String,

    #[serde(default)]
    pub vehicle_mods:  u32,
    #[serde(default)]
    pub extra_notes:  String,

    #[serde(default)]
    pub profiles: Vec<WeaponProfile>,

    #[serde(default)]
    pub cost: f32,
    #[serde(default)]
    pub weight: f32,

    #[serde(default)]
    pub abilities: Vec<String>,
}

impl Weapon {
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

    // pub fn import_from_id(
    //     &mut self,
    //     id: u32,
    //     available_data: &GameDataPackage,
    // ) {

    //     for weapon in available_data.weapons.iter() {
    //         if weapon.id == id {
    //             self.import_from_definition( weapon.id, &weapon );
    //             return;
    //         }
    //     }
    // }

    // pub fn import_from_definition(
    //     &mut self,
    //     id: u32,
    //     def: &Weapon,
    // ) {
    //     self = def.clone();
    // }

    pub fn get_ap_hr(&self) -> String {

        if self.profiles.len() > 0 {
            if self.profiles[0].ap > 0 {
                return "; AP ".to_owned() + &self.profiles[0].ap.to_string();
            }
        }
        "".to_owned()
    }

    pub fn get_reach_hr(&self) -> String {

        if self.profiles.len() > 0 {
            if self.profiles[0].reach > 0 {
                return "; Reach ".to_owned() + &self.profiles[0].reach.to_string();
            }
        }
        "".to_owned()
    }

    pub fn get_range_hr(&self) -> String {

        if self.profiles.len() > 0 {
            if self.profiles[0].melee_only {
                return "; Melee".to_owned();
            } else {
                return "; ".to_owned() + &self.profiles[0].range.to_owned();
            }
        } else {
            return "; No Range?".to_owned();
        }
    }

    pub fn get_damage_hr(&self) -> String {

        if self.profiles.len() > 0 {
            if self.profiles[0].add_strength_to_damage {
                return ("Str+".to_owned() + &self.profiles[0].damage).to_owned();
            } else {
                return self.profiles[0].damage.to_owned();
            }
        } else {
            return "No Damage?".to_owned();
        }
    }

    pub fn basic_info(&self) -> String {
        let mut rv = self.get_damage_hr();

        // rv += &"; ";
        rv += &self.get_range_hr();

        rv += &self.get_ap_hr();

        rv += &self.get_reach_hr();

        if !self.get_summary().is_empty() {
            rv += &"; ";
            rv += &self.get_summary();
        }

        return rv;
    }

    pub fn import_vars(&mut self, vars_option: &Option<WeaponVars>) {
        match vars_option {
            Some(vars) => {
                self.uuid = Uuid::parse_str(&vars.uuid).unwrap();
                self.custom_name = vars.custom_name.clone();
            }
            None => {}
        }
    }
}

impl Default for Weapon {
    fn default() -> Self {

        Weapon {
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
            abilities: Vec::new(),

            created_by_obj: None,
            deleted_by_obj: None,
            updated_by_obj: None,

            book_name: None,
            book_short_name: None,

            effects: Vec::new(),

            minimum_strength:  "".to_owned(),

            tw_cost: 0.0,
            tw_effects:  "".to_owned(),

            vehicle_mods:  0,
            extra_notes:  "".to_string(),

            profiles: Vec::new(),

            weight: 0.0,

            cost: 0.0,

        }

    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WeaponVars {
    #[serde(default)]
    pub custom_name: String,
    #[serde(default)]
    pub uuid: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct WeaponCombo {
    pub id: u32,
    #[serde(default, alias = "weaponOptions")]
    pub options: Option<WeaponVars>,
    #[serde(default)]
    pub def: Option<Weapon>,
}

impl Default for WeaponCombo {
    fn default() -> Self {
        WeaponCombo {
            id: 0,
            options: None,
            def: None,
        }
    }
}

#[derive(Deserialize, PartialEq, Serialize, Clone, Debug)]
pub struct WeaponProfile {
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
    pub heavy_weapon: bool,
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
    pub thrown_weapon: bool,

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

impl Default for WeaponProfile {
    fn default() -> Self {
        WeaponProfile {
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

            heavy_weapon: false,
            melee_only: false,
            counts_as_innate: false,
            notes:  "".to_string(),
            equipped: false,

            to_hit_mod:  0,

            damage_dice_base:  "".to_string(),
            damage_dice_base_plus:  0,

            is_shield: false,
            thrown_weapon: false,

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