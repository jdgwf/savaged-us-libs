use serde;
use serde::{Serialize, Deserialize};

use crate::public_user_info::PublicUserInfo;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum WebsocketMessageType {
    Online = 1,
    Offline = 2,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WebSocketMessage {
    pub token: String,
    pub kind: WebsocketMessageType,
    pub user: Option<PublicUserInfo>,
}