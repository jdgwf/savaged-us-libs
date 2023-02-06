use chrono::prelude::*;
use serde::{Deserialize, Serialize};


// use super::utils::bool_from_int_or_bool;
// use super::utils::float_to_int;
use crate::public_user_info::PublicUserInfo;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub enum HelpArticleSection {
    None = 0,
    Registration = 1,
    Characters = 2,
    Vehicles = 3,
    Saves = 4,
    Campaigns = 5,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct HelpArticle {
    pub id: u32,

    pub name: String,
    pub section: HelpArticleSection,
    pub tag: String,

    // #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub active: bool,

    #[serde(default)]
    pub copy: String,

    pub summary: String,

    // #[serde(deserialize_with = "bool_from_int_or_bool")]
    pub read_only: bool,

    pub created_by: u32,
    #[serde(default)]
    pub created_on: Option<DateTime<Utc>>,
    pub deleted_by: u32,
    #[serde(default)]
    pub deleted_on: Option<DateTime<Utc>>,
    pub updated_by: u32,
    #[serde(default)]
    pub updated_on: Option<DateTime<Utc>>,
    // pub version_of: u32,

    #[serde(default)]
    pub created_by_obj: Option<PublicUserInfo>,

    #[serde(default)]
    pub updated_by_obj: Option<PublicUserInfo>,

    #[serde(default)]
    pub deleted_by_obj: Option<PublicUserInfo>,
}

impl Default for HelpArticle {
    fn default() -> Self {
        HelpArticle {
            id: 0,
            name: "".to_owned(),
            section: HelpArticleSection::None,
            tag: "".to_owned(),
            copy: "".to_owned(),
            active: false,
            // deleted: false,
            summary: "".to_owned(),
            read_only: false,
            created_by: 0,
            created_on: Some(chrono::offset::Utc::now()),
            deleted_by: 0,
            deleted_on: None,
            updated_by: 0,
            updated_on: None,


            created_by_obj: None,
            deleted_by_obj: None,
            updated_by_obj: None,
        }
    }
}

impl HelpArticle {
    pub fn to_simple(&self) -> SimpleHelpArticle {
        SimpleHelpArticle {
            id: 0,
            name: self.name.to_owned(),
            section: self.section.to_owned(),
            tag: self.tag.to_owned(),
            copy: self.copy.to_owned(),
        }
    }
}

#[derive(Serialize, PartialEq, Deserialize, Clone, Debug)]
pub struct SimpleHelpArticle {
    pub id: u32,
    pub name: String,
    pub section: HelpArticleSection,
    pub tag: String,
    pub copy: String,
}

impl HelpArticleSection {
    pub fn as_str(&self) -> &str {
        match self {
            HelpArticleSection::None => {
                return "none";
            }
            HelpArticleSection::Registration => {
                return "registration";
            }
            HelpArticleSection::Characters => {
                return "characters";
            }
            HelpArticleSection::Saves => {
                return "saves";
            }
            HelpArticleSection::Campaigns => {
                return "campaigns";
            }
            HelpArticleSection::Vehicles => {
                return "vehicles";
            }
        }
    }
}
impl Default for HelpArticleSection {
    fn default() -> HelpArticleSection {
        HelpArticleSection::None
    }
}