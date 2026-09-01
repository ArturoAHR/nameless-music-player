use sqlx::{
    SqlitePool,
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};

pub async fn get_database_pool() -> SqlitePool {
    let connection_options = SqliteConnectOptions::new().in_memory(true);

    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect_with(connection_options)
        .await
        .expect("Could not create in-memory database");

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Could not run migrations");

    pool
}
