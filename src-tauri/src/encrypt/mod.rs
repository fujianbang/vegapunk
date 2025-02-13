use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// Derive key from password and salt
fn derive_key(password: &[u8], salt: SaltString) -> String {
    // Argon2id v19
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password, &salt)
        .unwrap()
        .to_string();

    password_hash
}

/// Verify password
fn verify_password(password_hash: &str, password: &[u8]) -> bool {
    let parsed_hash = PasswordHash::new(password_hash).unwrap();

    Argon2::default()
        .verify_password(password, &parsed_hash)
        .is_ok()
}

fn random_salt() -> SaltString {
    SaltString::generate(&mut OsRng)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_verify_password() {
        let salt = random_salt();
        let key = derive_key(b"password", salt);
        assert!(verify_password(&key, b"password"));
    }
}
