use serde;
use serde::{Deserialize, Serialize};

use super::gear::Gear;
use super::weapon::Weapon;
use super::armor::Armor;


#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ContainerItem {
    cost_buy: i32,
    custom_def_gear: Option<Gear>,
    custom_def_weapon: Option<Weapon>,
    custom_def_armor: Option<Armor>,
    id: i32,
    name: String,
    count_current: i32,
    container: bool,
    total_cost_buy: i32,
    total_weight: i32,
    item_type: String,
    weight: i32,
    weight_display: String,
    scifi_mod: Option<String>,
    setting_item: bool,
    uuid: String,
    contains: Vec<ContainerItem>,
}