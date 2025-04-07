use serde::Serialize;

#[derive(Serialize)]
pub struct Habit {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}
