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
/// Separate password hash and hashing logic from User struct
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

    pub fn login(&self, password: String) -> bool {
        check_password(password, &self.hash)
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

fn hash_password(password: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(password)
}

fn check_password(password: String, hash: &String) -> bool {
    if password == *hash {
        return true;
    }
    false
}
