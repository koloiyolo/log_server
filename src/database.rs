pub async fn init_database(database_url: &String) {
    let pool = sqlx::SqlitePool::connect(database_url).await.unwrap();

    let _ = sqlx::query_file!("migrations/message.sql")
        .execute(&pool)
        .await;

    let _ = sqlx::query_file!("migrations/user.sql")
        .execute(&pool)
        .await;
}
