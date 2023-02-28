use crate::utils::get_dice_value::get_dice_value;
use serde;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Attributes {
    pub selected_agility: u8,
    pub selected_smarts: u8,
    pub selected_spirit: u8,
    pub selected_strength: u8,
    pub selected_vigor: u8,

    pub boosted_agility: u8,
    pub boosted_smarts: u8,
    pub boosted_spirit: u8,
    pub boosted_strength: u8,
    pub boosted_vigor: u8,

    pub bonus_agility: i8,
    pub bonus_smarts: i8,
    pub bonus_spirit: i8,
    pub bonus_strength: i8,
    pub bonus_vigor: i8,

    pub min_agility: u8,
    pub min_smarts: u8,
    pub min_spirit: u8,
    pub min_strength: u8,
    pub min_vigor: u8,

    pub max_agility: u8,
    pub max_smarts: u8,
    pub max_spirit: u8,
    pub max_strength: u8,
    pub max_vigor: u8,
}
impl Default for Attributes {
    fn default() -> Attributes {
        Attributes {
            selected_agility: 0,
            boosted_agility: 0,
            bonus_agility: 0,

            selected_smarts: 0,
            boosted_smarts: 0,
            bonus_smarts: 0,

            selected_spirit: 0,
            boosted_spirit: 0,
            bonus_spirit: 0,

            selected_strength: 0,
            boosted_strength: 0,
            bonus_strength: 0,

            selected_vigor: 0,
            boosted_vigor: 0,
            bonus_vigor: 0,

            min_agility: 0,
            min_smarts: 0,
            min_spirit: 0,
            min_strength: 0,
            min_vigor: 0,

            max_agility: 4,
            max_smarts: 4,
            max_spirit: 4,
            max_strength: 4,
            max_vigor: 4,
        }
    }
}
impl Attributes {

    pub fn agility_hr(&self) -> String {
        get_dice_value(
            self.selected_agility + self.boosted_agility,
            self.bonus_agility,
        )
        .clone()
    }

    pub fn smarts_hr(&self) -> String {
        get_dice_value(
            self.selected_smarts + self.boosted_smarts,
            self.bonus_smarts,
        )
        .clone()
    }

    pub fn spirit_hr(&self) -> String {
        get_dice_value(
            self.selected_spirit + self.boosted_spirit,
            self.bonus_spirit,
        )
        .clone()
    }

    pub fn strength_hr(&self) -> String {
        get_dice_value(
            self.selected_strength + self.boosted_strength,
            self.bonus_strength,
        )
        .clone()
    }

    pub fn vigor_hr(&self) -> String {
        get_dice_value(self.selected_vigor + self.boosted_vigor, self.bonus_vigor).clone()
    }

    pub fn set_attribute_boosted_agility(&mut self, new_val: u8) {
        self.min_agility = new_val + 1;
        self.max_agility = new_val + 5;
        self.boosted_agility = new_val;
    }
    pub fn set_attribute_boosted_smarts(&mut self, new_val: u8) {
        self.min_smarts = new_val + 1;
        self.max_smarts = new_val + 5;
        self.boosted_smarts = new_val;
    }
    pub fn set_attribute_boosted_spirit(&mut self, new_val: u8) {
        self.min_spirit = new_val + 1;
        self.max_spirit = new_val + 5;
        self.boosted_spirit = new_val;
    }
    pub fn set_attribute_boosted_strength(&mut self, new_val: u8) {
        self.min_strength = new_val + 1;
        self.max_strength = new_val + 5;
        self.boosted_strength = new_val;
    }
    pub fn set_attribute_boosted_vigor(&mut self, new_val: u8) {
        self.min_vigor = new_val + 1;
        self.max_vigor = new_val + 5;
        self.boosted_vigor = new_val;
    }

    pub fn set_attribute_bonus_agility(&mut self, new_val: i8) {
        self.bonus_agility = new_val;
    }
    pub fn set_attribute_bonus_smarts(&mut self, new_val: i8) {
        self.bonus_smarts = new_val;
    }
    pub fn set_attribute_bonus_spirit(&mut self, new_val: i8) {
        self.bonus_spirit = new_val;
    }
    pub fn set_attribute_bonus_strength(&mut self, new_val: i8) {
        self.bonus_strength = new_val;
    }
    pub fn set_attribute_bonus_vigor(&mut self, new_val: i8) {
        self.bonus_vigor = new_val;
    }

    pub fn reset(&mut self) {
        self.boosted_agility = 0;
        self.boosted_smarts = 0;
        self.boosted_spirit = 0;
        self.boosted_strength = 0;
        self.boosted_vigor = 0;

        self.bonus_agility = 0;
        self.bonus_smarts = 0;
        self.bonus_spirit = 0;
        self.bonus_strength = 0;
        self.bonus_vigor = 0;
    }
}
