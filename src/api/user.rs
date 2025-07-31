use poem::{Result, web::Data};
use poem_openapi::{
    Object, OpenApi,
    param::Path,
    payload::{Json, PlainText},
};
use sqlx::SqlitePool;

use crate::{encryption::hash_password, model::user::User};

type UserResponse = Result<Json<Vec<User>>>;

#[derive(Object)]
pub struct CreateRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Object)]
pub struct UpdateRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Object)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

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

    #[oai(path = "/user/:id", method = "get")]
    async fn get(&self, pool: Data<&SqlitePool>, id: Path<i64>) -> Result<Json<User>> {
        let result = sqlx::query_file_as!(User, "sql/user/get.sql", id.0)
            .fetch_one(pool.0)
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
            .unwrap_or(0);

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

    #[oai(path = "/user/update/:id", method = "post")]
    async fn update(
        &self,
        pool: Data<&SqlitePool>,
        id: Path<i64>,
        body: Json<UpdateRequest>,
    ) -> Json<i64> {
        let rowid = id.0;
        let username = &body.username;
        let email = &body.email;
        let password = &body.password;

        let hash = match password {
            Some(password) => match hash_password(password.to_owned()) {
                Ok(hash) => Some(hash),
                Err(_) => None,
            },
            None => None,
        };

        let _ = sqlx::query_file!("sql/user/update.sql", username, email, hash, rowid)
            .execute(pool.0)
            .await
            .unwrap();

        Json(rowid)
    }

    #[oai(path = "/user/delete/:id", method = "post")]
    async fn delete(&self, pool: Data<&SqlitePool>, id: Path<i64>) -> Json<bool> {
        let rowid = id.0;
        let result = sqlx::query_file!("sql/user/delete.sql", rowid)
            .execute(pool.0)
            .await
            .is_ok();
        Json(result)
    }

    #[oai(path = "/user/login", method = "post")]
    async fn login(&self, pool: Data<&SqlitePool>, body: Json<LoginRequest>) -> Json<i64> {
        let username = &body.username;
        let password = &body.password;
        let user = sqlx::query_file_as!(User, "sql/user/login.sql", username)
            .fetch_one(pool.0)
            .await
            .unwrap();

        // 0 means no user. No data in sqlite table can have rowid of 0
        Json(user.login(password.to_owned()).unwrap_or(0))
    }
}
