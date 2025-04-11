use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Habit {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}
#[derive(Serialize, Deserialize)]
pub struct CreateHabit {
    pub title: String,
    pub completed: bool,
}
