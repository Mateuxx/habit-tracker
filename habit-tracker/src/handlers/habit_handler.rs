use std::sync::Arc;

use crate::models::habit::CreateHabit;
use crate::models::{ habit::Habit, state::AppState };
use axum::debug_handler;
use axum::{ Json, extract::State, http::StatusCode };

//Returns a list of mock habits
pub async fn list_habits(State(state): State<Arc<AppState>>) -> (StatusCode, Json<Vec<Habit>>) {
    let habits = state.habits.lock().unwrap(); //unlock this so we can useee
    (StatusCode::OK, Json(habits.clone()))
}

// post Handler - returns a list
#[debug_handler]
pub async fn create_habit(
    //extraindo aqui as coisas, o estado e o payload e
    // aqui eu faço a extração do state que está sendo passado pela aplicação e atribuo o valor a variavl: state
    // Digo que ela tem que ser do tipo: State<Arc<AppState>>
    State(state): State<Arc<AppState>>,
    //extraactor do json e eu estou dizendo que eu quero que ele tenha desustrado para um CreateHabit
    //aqui eu estraio o json do tipo CreateHabit e coloco na variavel payload
    Json(payload): Json<CreateHabit>
) -> (StatusCode, Json<Habit>) {
    let mut habits = state.habits.lock().unwrap();

    let new_habit = Habit {
        id: (habits.len() + 1) as u32,
        title: payload.title,
        completed: payload.completed,
    };

    //this is for save on the state
    habits.push(new_habit.clone());

    //this is like a return with the
    (StatusCode::CREATED, Json(new_habit))
}
