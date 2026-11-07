use argon2::password_hash::Error;
use argon2::{
    Argon2, PasswordHash,
    password_hash::{PasswordHasher, PasswordVerifier},
};

pub(crate) fn hash_password(password: &str) -> Result<String, Error> {
    let argon2 = Argon2::default();
    Ok(argon2.hash_password(password.as_bytes())?.to_string())
}

pub(crate) fn verify_password(password: &str, stored_hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(stored_hash) {
        Ok(h) => h,
        Err(_) => return false,
    };

    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[cfg(test)]
mod tests {
    use crate::utils::password::{hash_password, verify_password};

    #[test]
    fn hash_and_verify() {
        // given
        let password = "password".to_string();
        let password_hashed = hash_password(&password).unwrap();

        // when
        let actual = verify_password(password.as_str(), &password_hashed);

        // then
        assert_eq!(true, actual);
    }
}
