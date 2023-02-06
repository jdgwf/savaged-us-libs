use serde;
use serde::{Deserialize, Serialize};

use super::gear::Gear;
use super::weapon::Weapon;
use super::armor::Armor;

#[derive(Deserialize, PartialEq, Serialize, Clone, Debug)]
pub struct ContainerItem {
    pub cost_buy: i32,
    pub custom_def_gear: Option<Gear>,
    pub custom_def_weapon: Option<Weapon>,
    pub custom_def_armor: Option<Armor>,
    pub id: i32,
    pub name: String,
    pub count_current: i32,
    pub container: bool,
    pub total_cost_buy: i32,
    pub total_weight: i32,
    pub item_type: String,
    pub weight: i32,
    pub weight_display: String,
    pub scifi_mod: Option<String>,
    pub setting_item: bool,
    pub uuid: String,
    pub contains: Vec<ContainerItem>,
}