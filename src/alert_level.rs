use serde::{Deserialize, Serialize};
#[derive(Serialize, PartialEq, Deserialize, Clone, Debug)]
pub enum AlertLevel {
    Secondary = 0,
    Info = 1,
    Warning = 2,
    Danger = 3,
    Success = 4,
}
