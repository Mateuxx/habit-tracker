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
//Struct to handle the update habit
//option is for if the client send only what he wants to change 
//maybe he cannot want to update all the elements
#[derive(Deserialize)]
pub struct UpdateHabit {
    pub title: Option<String>,
    pub completed: Option<bool>,
}
