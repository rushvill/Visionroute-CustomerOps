//! Argon2id password hashing.

use argon2::password_hash::{
    rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
};
use argon2::Argon2;

use crate::error::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    if password.len() < 10 {
        return Err(AppError::Validation(
            "Password must be at least 10 characters.".to_owned(),
        ));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|error| {
            tracing::error!(%error, "password hashing failed");
            AppError::Internal(anyhow::anyhow!("password hashing failed"))
        })?;

    Ok(hash.to_string())
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let parsed = PasswordHash::new(password_hash).map_err(|error| {
        tracing::error!(%error, "stored password hash is invalid");
        AppError::Internal(anyhow::anyhow!("stored password hash is invalid"))
    })?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::{hash_password, verify_password};

    #[test]
    fn round_trip_argon2id() {
        let hash = hash_password("VisionRouteDemo26!").expect("hash");
        assert!(verify_password("VisionRouteDemo26!", &hash).expect("verify"));
        assert!(!verify_password("wrong-password", &hash).expect("verify wrong"));
    }

    #[test]
    fn rejects_short_password() {
        assert!(hash_password("short").is_err());
    }
}
