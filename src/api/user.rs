use poem::{Result, web::Data};
use poem_openapi::{
    OpenApi,
    payload::{Json, PlainText},
};
use sqlx::SqlitePool;

use crate::model::user::User;

type UserResponse = Result<Json<Vec<User>>>;

pub struct UserApi;

#[OpenApi]
impl UserApi {
    #[oai(path = "/user/all", method = "get")]
    async fn get_all(&self, pool: Data<&SqlitePool>) -> UserResponse {
        let result = sqlx::query_file_as!(User, "sql/user/select.sql")
            .fetch_all(pool.0)
            .await
            .unwrap();
        Ok(Json(result))
    }

    #[oai(path = "/user/search", method = "post")]
    async fn search(&self, pool: Data<&SqlitePool>, query: PlainText<String>) -> UserResponse {
        let query = format!("%{}%", query.0);
        let result = sqlx::query_file_as!(User, "sql/user/search.sql", query)
            .fetch_all(pool.0)
            .await
            .unwrap();
        Ok(Json(result))
    }

    #[oai(path = "/user/count", method = "get")]
    async fn count(&self, pool: Data<&SqlitePool>) -> Json<i32> {
        let result = sqlx::query_file_scalar!("sql/user/count.sql")
            .fetch_one(pool.0)
            .await
            .unwrap();

        Json(result)
    }
}
