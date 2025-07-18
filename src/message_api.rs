use poem::{Result, web::Data};
use poem_openapi::{
    OpenApi,
    payload::{Json, PlainText},
};
use sqlx::SqlitePool;

use super::message::Message;

type MessageResponse = Result<Json<Vec<Message>>>;

pub struct MessageApi;

#[OpenApi]
impl MessageApi {
    #[oai(path = "/messages", method = "get")]
    async fn get_all(&self, pool: Data<&SqlitePool>) -> MessageResponse {
        let result = sqlx::query_file_as!(Message, "sql/message/select.sql")
            .fetch_all(pool.0)
            .await
            .unwrap();

        Ok(Json(result))
    }

    #[oai(path = "/count", method = "get")]
    async fn count(&self, pool: Data<&SqlitePool>) -> Json<i32> {
        let result = sqlx::query_file_scalar!("sql/message/count.sql")
            .fetch_one(pool.0)
            .await
            .unwrap();

        Json(result)
    }

    #[oai(path = "/search", method = "post")]
    async fn search(&self, pool: Data<&SqlitePool>, query: PlainText<String>) -> MessageResponse {
        println!("{}", &query.0);
        let query = format!("%{}%", query.0);
        let result = sqlx::query_file_as!(Message, "sql/message/search.sql", query)
            .fetch_all(pool.0)
            .await
            .unwrap();

        Ok(Json(result))
    }
}
