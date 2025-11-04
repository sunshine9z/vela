use crate::commons::error::UserDomainError;

pub trait PwdEncryptTrait {
    fn encrypt(&self, password: &String) -> Result<String, UserDomainError>;
    fn verify(&self, password: &String, encrypted_pwd: &String) -> bool;
}
