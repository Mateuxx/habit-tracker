use std::sync::Arc;

use crate::models::habit::{ CreateHabit, UpdateHabit };
use crate::models::{ habit::Habit, state::AppState };
use axum::debug_handler;
use axum::extract::Path;
use axum::{ Json, extract::State, http::StatusCode };
use sqlx::query_as;
use uuid::Uuid;

//we share the database state with all the logic d
pub async fn list_habits(State(state): State<Arc<AppState>>) -> Result<
    (StatusCode, Json<Vec<Habit>>),
    (StatusCode, String)
> {
    let query = "
    SELECT * FROM habits  ORDER BY created_at DESC";

    let result = query_as::<_, Habit>(query).fetch_all(&state.db).await;

    match result {
        Ok(habits) => Ok((StatusCode::OK, Json(habits))),
        Err(err) => {
            eprintln!("Erro ao buscar hábitos: {:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Erro ao buscar hábitos".into()))
        }
    }
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
) -> Result<(StatusCode, Json<Habit>), (StatusCode, String)> {
    let query_result = query_as::<_, Habit>(
        r#"
        INSERT INTO habits (id, title, completed)
        VALUES ($1, $2, $3)
        RETURNING *
        "#
    )
        //Bind a value for use with this SQL query.
        .bind(Uuid::new_v4()) // gera novo id
        .bind(&payload.title) // insere o título
        .bind(payload.completed) // insere o completed
        .fetch_one(&state.db).await; // executa e espera 1 resultado

    match query_result {
        Ok(habit) => Ok((StatusCode::CREATED, Json(habit))),
        Err(err) => {
            eprintln!("Erro ao criar hábito: {:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Erro ao criar hábito".into()))
        }
    }
}
pub async fn update_habit(
    Path(id): Path<Uuid>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<UpdateHabit>
) -> Result<(StatusCode, Json<Habit>), (StatusCode, String)> {
    let query =
        "UPDATE habits
     SET title = $1, completed = $2, updated_at = NOW()
     WHERE id = $3
     RETURNING *";

    let query_result = query_as::<_, Habit>(query)
        // this is related to $1
        .bind(&payload.title)
        // this is related to $2
        .bind(&payload.completed)
        // this is related to $3
        .bind(&id) // Use the extracted `id` from the path
        .fetch_optional(&state.db).await; // returns and Option<Habit >

    match query_result {
        Ok(Some(habit)) => Ok((StatusCode::OK, Json(habit))),
        // Não foi econtrado alguma coisa nele mas tudo certinho
        Ok(None) => Err((StatusCode::NOT_FOUND, "Hábito não encontrado".into())),
        Err(err) => {
            eprintln!("Erro ao atualizar hábito: {:?}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, "Erro ao atualizar hábito".into()))
        }
    }
}
