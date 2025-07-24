use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use password_hash::{PasswordHash, SaltString, rand_core::OsRng};
use poem_openapi::Object;
use std::fmt;

#[derive(Debug, Clone, Object)]
pub struct User {
    pub rowid: i64,
    pub username: String,
    pub email: String,
    pub hash: String,
}
/// TODO
/// Example implementation:
///
/// Separate Hash struct that holds hashes tied to user username or rowid
/// Used only for login, user creation, password reset.
impl User {
    pub fn new(username: String, email: String, password: String) -> Self {
        if let Ok(hash) = hash_password(password) {
            return User {
                rowid: 0,
                username,
                email,
                hash,
            };
        }
        todo!("Handle gracefully, failed to create user");
    }

    pub fn login(&self, password: String) -> Result<i64, String> {
        match check_password(&password, &self.hash) {
            true => Ok(*&self.rowid),
            false => Err("".to_string()),
        }
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "(Id: {}\nUsername: {}\nEmail: {})",
            self.rowid, self.username, self.email
        )
    }
}

fn hash_password(password: String) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    let hash = argon2.hash_password(&password.into_bytes(), &salt)?;
    Ok(hash.to_string())
}

fn check_password(password: &String, hash: &String) -> bool {
    let hash = PasswordHash::new(hash);
    if let Ok(hash) = hash {
        Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .is_ok()
    } else {
        false
    }
}

#[derive(Object)]
pub struct CreateRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Object)]
pub struct UpdateRequest {
    pub username: String,
    pub email: String,
}

#[derive(Object)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
