use serde;
use serde::{Deserialize, Serialize};
// use uuid::Uuid;
use chrono::prelude::*;

use crate::player_character::game_data_package::GameDataPackage;
use crate::public_user_info::PublicUserInfo;
// use crate::public_user_info::PublicUserInfo;
use crate::save_db_row::SaveDBRow;
use crate::user::User;
use crate::web_content::WebContent;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum WebsocketMessageType {
    Online = 1,
    Offline = 2,
    GameDataPackageUpdated = 3,
    SavesUpdated = 4,
    Logout = 5,
    SetLocation = 6,
    SetRoom = 7,
    RequestUsers = 8,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebSocketMessage {
    pub token: Option<String>,
    pub kind: WebsocketMessageType,
    pub user: Option<User>,
    pub payload: Option<String>,
    pub updated_on: Option<DateTime<Utc>>,
    pub game_data: Option<GameDataPackage>,
    pub saves: Option<Vec<SaveDBRow>>,
    pub include_saves: bool,
    pub user_list: Option<Vec<PublicUserInfo>>,
    pub web_content: Option<WebContent>,
}

impl Default for WebSocketMessage {
    fn default() -> Self {
        WebSocketMessage {
            kind: WebsocketMessageType::SavesUpdated,
            token: None,
            user: None,
            payload: None,
            updated_on: None,
            game_data: None,
            saves: None,
            user_list: None,
            include_saves: false,
            web_content: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SimpleAPIReturn {
    pub success: bool,
    pub message: String,
}

