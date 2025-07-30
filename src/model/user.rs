use poem_openapi::Object;
use std::fmt;

use crate::encryption::{check_password, hash_password};

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
