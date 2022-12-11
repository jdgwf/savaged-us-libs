use serde;
use serde::{Serialize, Deserialize};

use chrono::prelude::*;

use crate::player_character::chargen_data::ChargenData;
use crate::public_user_info::PublicUserInfo;
use crate::save_db_row::SaveDBRow;
use crate::user::User;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum WebsocketMessageType {
    Online = 1,
    Offline = 2,
    ChargenData = 3,
    Saves = 4,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebSocketMessage {
    pub token: Option<String>,
    pub kind: WebsocketMessageType,
    pub user: Option<User>,
    pub payload: Option<String>,
    pub updated_on: Option<DateTime<Utc>>,
    pub chargen_data: Option<ChargenData>,
    pub saves: Option<Vec<SaveDBRow>>,
}