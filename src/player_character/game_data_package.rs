use serde::{Deserialize, Serialize};

use crate::book::Book;

use super::edge::Edge;
use super::hindrance::Hindrance;

use super::armor::Armor;
use super::gear::Gear;
use super::weapon::Weapon;

#[derive(Deserialize, PartialEq, Serialize, Clone, Debug)]
pub struct GameDataPackage {
    pub data_level: GameDataPackageLevel,
    pub books: Vec<Book>,
    pub hindrances: Vec<Hindrance>,
    pub edges: Vec<Edge>,

    pub gear: Vec<Gear>,
    pub armor: Vec<Armor>,
    pub weapons: Vec<Weapon>,
}

impl Default for GameDataPackage {
    fn default() -> Self {
        GameDataPackage {
            data_level: GameDataPackageLevel::Unloaded,
            books: Vec::new(),
            hindrances: Vec::new(),
            edges: Vec::new(),

            gear: Vec::new(),
            armor: Vec::new(),
            weapons: Vec::new(),
        }
    }
}

#[derive(Serialize, PartialEq, Deserialize, Clone, Debug)]
pub enum GameDataPackageLevel {
    Unloaded = 0,
    Anonymous = 1,
    Registered = 2,
    WildCard = 3,
    Developer = 4,
    Admin = 5,
}
