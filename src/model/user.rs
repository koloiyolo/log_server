use poem_openapi::Object;

#[derive(Debug, Clone, Object)]
pub struct User {
    pub rowid: i64,
    pub username: String,
    pub email: String,
    pub hash: String,
}

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

fn hash_password(password: String) -> Result<String, Box<dyn std::error::Error>> {
    Ok(password)
}

fn check_password(password: String, hash: &String) -> bool {
    if password == *hash {
        return true;
    }
    false
}
