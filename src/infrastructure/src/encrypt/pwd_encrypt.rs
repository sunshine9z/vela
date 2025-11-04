use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};
use user_domain::{commons::error::UserDomainError, repository::encrypt::PwdEncryptTrait};

pub struct PwdEncryptImpl {}

impl PwdEncryptTrait for PwdEncryptImpl {
    fn encrypt(&self, password: &String) -> Result<String, UserDomainError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|_| UserDomainError::AuthError("生成密码哈希失败".to_string()))?
            .to_string();
        Ok(password_hash)
    }

    fn verify(&self, password: &String, encrypted_pwd: &String) -> bool {
        let parsed_hash = match PasswordHash::new(encrypted_pwd) {
            Ok(h) => h,
            Err(_) => return false,
        };
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}
