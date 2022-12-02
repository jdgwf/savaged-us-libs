use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct PublicUserInfo {
    pub username: String,
    pub name: String,
    pub twitter: String,
    pub image: String,
    pub r#type: String,
    pub page: String,
    pub banned: bool,
    pub banned_reason: String,
    pub show_user_page: bool,
    pub user_page_url: String,
    pub bio: Vec<String>,
    pub shares: Vec<String>,
    pub id: u32,
    pub room_id: String,

    pub shared_saves: Vec<String>,
}


impl Default for PublicUserInfo {
    fn default() -> Self {
        PublicUserInfo{
            username: "".to_owned(),
            name: "".to_owned(),
            twitter: "".to_owned(),
            image: "".to_owned(),
            r#type: "".to_owned(),
            page: "".to_owned(),
            banned: false,
            banned_reason: "".to_owned(),
            show_user_page: false,
            user_page_url: "".to_owned(),
            bio: Vec::new(),
            shares: Vec::new(),
            id: 0,
            room_id: "".to_owned(),
            shared_saves: Vec::new(),
        }
    }

}