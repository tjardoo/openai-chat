use sqlx::MySqlPool;

pub struct AppState {
    pub pool: MySqlPool,
}
