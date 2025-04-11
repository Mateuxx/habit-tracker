use serde::{ Deserialize, Serialize };

#[derive(Clone, Serialize, Debug)]
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
