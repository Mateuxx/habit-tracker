use serde::{ Deserialize, Serialize };
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Clone, Serialize, Debug, FromRow)]
pub struct Habit {
    pub id: Uuid,
    pub title: String,
    pub completed: bool,
}
#[derive(Serialize, Deserialize)]
pub struct CreateHabit {
    pub title: String,
    pub completed: bool,
}
//Struct to handle the update habit
//option is for if the client send only what he wants to change
//maybe he cannot want to update all the elements
#[derive(Deserialize)]
pub struct UpdateHabit {
    pub title: Option<String>,
    pub completed: Option<bool>,
}
