use argon2::{Argon2, PasswordHasher, PasswordVerifier};
use password_hash::{PasswordHash, SaltString, rand_core::OsRng};

pub fn hash_password(password: String) -> Result<String, password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2.hash_password(&password.into_bytes(), &salt)?;
    Ok(hash.to_string())
}

pub fn check_password(password: &str, hash: &str) -> bool {
    let hash = PasswordHash::new(hash);
    if let Ok(hash) = hash {
        Argon2::default()
            .verify_password(password.as_bytes(), &hash)
            .is_ok()
    } else {
        false
    }
}
