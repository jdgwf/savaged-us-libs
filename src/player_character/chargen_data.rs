use serde::{Deserialize, Serialize};

use crate::book::Book;

use super::hindrance::Hindrance;
use super::edge::Edge;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ChargenData {
    pub data_level: ChargenDataLevel,
    pub books: Vec<Book>,
    pub hindrances: Vec<Hindrance>,
    pub edges: Vec<Edge>,
}

impl Default for ChargenData {
    fn default( ) -> Self {
        ChargenData {
            data_level: ChargenDataLevel::Unloaded,
            books: Vec::new(),
            hindrances: Vec::new(),
            edges: Vec::new(),
        }
    }

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ChargenDataLevel {
    Unloaded = 0,
    Anonymous = 1,
    Registered = 2,
    WildCard = 3,
    Developer = 4,
    Admin = 5,

}