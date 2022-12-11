use serde::{Deserialize, Serialize};

use crate::book::Book;

use super::hindrance::Hindrance;
use super::edge::Edge;

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq)]
pub struct ChargenData {
    pub books: Vec<Book>,
    pub hindrances: Vec<Hindrance>,
    pub edges: Vec<Edge>,
}