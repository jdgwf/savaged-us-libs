use serde::{Deserialize, Serialize};

use crate::{help_article::SimpleHelpArticle, partner::SimplePartner, banner::SimpleBanner, announcement::SimpleAnnouncement};


#[derive(Serialize, PartialEq, Deserialize, Clone, Debug)]
pub struct WebContent {
    pub partners: Option<Vec<SimplePartner>>,
    pub banners: Option<Vec<SimpleBanner>>,
    pub announcements: Option<Vec<SimpleAnnouncement>>,
    pub help_articles: Option<Vec<SimpleHelpArticle>>,
}