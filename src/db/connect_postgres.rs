use sqlx::postgres::PgPoolOptions;

pub async fn connect_pg(database_url: String) -> sqlx::PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres")
}
