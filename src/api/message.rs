use poem::{Result, web::Data};
use poem_openapi::{
    OpenApi,
    payload::{Json, PlainText},
};
use sqlx::SqlitePool;

use crate::model::dbrecord::DbRecord;
use crate::model::message::Message;

type MessageResponse = Result<Json<Vec<Message>>>;

pub struct MessageApi;

#[OpenApi]
impl MessageApi {
    #[oai(path = "/message", method = "get")]
    async fn get_all(&self, pool: Data<&SqlitePool>) -> MessageResponse {
        let result = Message::get_limit(1, pool.0).await.unwrap();

        Ok(Json(result))
    }

    #[oai(path = "/message/search", method = "post")]
    async fn search(&self, pool: Data<&SqlitePool>, query: PlainText<String>) -> MessageResponse {
        let query = format!("%{}%", query.0);
        let result = Message::search(query, pool.0).await.unwrap();
        Ok(Json(result))
    }

    #[oai(path = "/message/count", method = "get")]
    async fn count(&self, pool: Data<&SqlitePool>) -> Json<i32> {
        let result = sqlx::query_file_scalar!("sql/message/count.sql")
            .fetch_one(pool.0)
            .await
            .unwrap();

        Json(result)
    }
}
