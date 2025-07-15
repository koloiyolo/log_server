use poem::{Result, web::Data};
use poem_openapi::{OpenApi, payload::Json};
use sqlx::SqlitePool;

use super::message::Message;

type MessageResponse = Result<Json<Vec<Message>>>;

pub struct MessageApi;

#[OpenApi]
impl MessageApi {
    #[oai(path = "/messages", method = "get")]
    async fn get_all(&self, pool: Data<&SqlitePool>) -> MessageResponse {
        let result = sqlx::query_as!(Message, "SELECT * FROM message")
            .fetch_all(pool.0)
            .await
            .unwrap();

        Ok(Json(result))
    }

    #[oai(path = "/count", method = "get")]
    async fn count(&self, pool: Data<&SqlitePool>) -> Json<i32> {
        let result = sqlx::query_scalar!("SELECT COUNT(subject) FROM message")
            .fetch_one(pool.0)
            .await
            .unwrap();

        Json(result)
    }
}
