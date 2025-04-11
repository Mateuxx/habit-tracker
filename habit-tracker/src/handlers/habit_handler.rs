use axum::{ http::StatusCode, Json };
use crate::models::habit::{ CreateHabit, Habit };

//Returns a list of mock habits
pub async fn list_habits() -> (StatusCode, Json<Vec<Habit>>) {
    let habits = vec![
        Habit {
            id: 1,
            title: "Beber água".to_string(),
            completed: false,
        },
        Habit {
            id: 2,
            title: "Caminhar e Cardio de 30".to_string(),
            completed: false,
        }
    ];

    (StatusCode::OK, Json(habits))
}

// post Handler - returns a list

pub async fn create_habit_handler(
    //Axum receivies the request and tries to convert to a instance of CreateHabit
    Json(payload): Json<CreateHabit>
) -> (StatusCode, Json<Habit>) {
    let new_habit = Habit {
        id: 3,
        title: payload.title,
        completed: payload.completed,
    };

    (StatusCode::CREATED, Json(new_habit))
}
