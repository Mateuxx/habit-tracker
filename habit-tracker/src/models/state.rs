use std::sync::Mutex;
use crate::models::habit::Habit;

#[derive(Debug)]
pub struct AppState {
    pub habits: Mutex<Vec<Habit>>,
}
