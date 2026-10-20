use argon2::{Argon2, PasswordHash};
use password_hash::{Error, PasswordHasher, PasswordVerifier};

pub fn hash_password(password: &str) -> Result<String, Error> {
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes())?
        .to_string();
    Ok(password_hash)
}

pub fn verify_password(password: &str, stored_hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(stored_hash) {
        Ok(h) => h,
        Err(_) => return false, // フォーマットが不正な場合は検証失敗
    };

    // 2. 入力されたパスワードを同じ条件でハッシュ化し、保存されていたハッシュと比較
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[cfg(test)]
mod tests {
    mod hash_and_verify {
        use password_hash::Error;
        use crate::utils::hash::{hash_password, verify_password};

        #[test]
        fn success() -> Result<(), Error> {
            // given
            let password = "p@ssw0rd";

            // when
            let actual = verify_password(password,&hash_password(password)?);

            // then
            assert!(actual);
            Ok(())
        }
    }
}
