use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct HiddenBanner {
    pub src: String,
    pub id: u32,
    pub label: String,
}
