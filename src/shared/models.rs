use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct LoginBonus {
    id: i32,
    title: String,
    description: String,
}

#[derive(Serialize, Deserialize)]
pub struct DailyMission {
    id: i32,
    title: String,
    description: String,
    reward: i32,
    is_completed: bool,
}
