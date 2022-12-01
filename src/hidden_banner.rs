use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]

pub struct HiddenBanner {
    pub src: String,
    pub id: u32,
    pub label: String,
}