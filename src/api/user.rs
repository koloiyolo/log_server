use poem::{Result, web::Data};
use poem_openapi::{
    OpenApi,
    payload::{Json, PlainText},
};
use sqlx::SqlitePool;

use crate::model::user::{CreateRequest, LoginRequest, User};

type UserResponse = Result<Json<Vec<User>>>;

pub struct UserApi;

#[OpenApi]
impl UserApi {
    #[oai(path = "/user", method = "get")]
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

    #[oai(path = "/user/create", method = "post")]
    async fn create(&self, pool: Data<&SqlitePool>, body: Json<CreateRequest>) -> Json<i64> {
        let username = &body.username;
        let email = &body.email;
        let password = &body.password;

        let user = User::new(username.to_owned(), email.to_owned(), password.to_owned());

        let result = sqlx::query_file!("sql/user/insert.sql", user.username, user.email, user.hash)
            .execute(pool.0)
            .await
            .unwrap()
            .last_insert_rowid();
        Json(result)
    }

    #[oai(path = "/user/login", method = "post")]
    async fn login(&self, pool: Data<&SqlitePool>, body: Json<LoginRequest>) -> Json<bool> {
        let username = &body.username;
        let password = &body.password;
        let user = sqlx::query_file_as!(User, "sql/user/get.sql", username)
            .fetch_one(pool.0)
            .await
            .unwrap();
        Json(user.login(password.to_owned()))
    }
}
