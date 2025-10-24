use crate::encrypt::pwd_encrypt::PwdEncryptImpl;
use crate::persistence::user_repo::Model as UserModel;
use crate::{cache::CacheManager, web_error, web_info};
use async_trait::async_trait;
use commonx::error::AppError;
use user_domain::{
    UserDomainImpl,
    commons::error::UserDomainError,
    entity::captcha::CaptchaCacheInfo,
    new_user_domain,
    repository::{cache::CacheRepositoryTrait, user::UserRepositoryTrait},
};

pub struct UserDomainRepositoryImpl {}

#[async_trait]
impl UserRepositoryTrait for UserDomainRepositoryImpl {
    async fn get_by_username(
        &self,
        username: String,
    ) -> Result<Option<user_domain::entity::user::User>, UserDomainError> {
        UserModel::find_by_username(username.as_str())
            .await
            .map_or_else(
                |e| Err(UserDomainError::DbError(e.to_string())),
                |user| Ok(user.map(|u| u.into())),
            )
    }
}

impl From<UserModel> for user_domain::entity::user::User {
    fn from(user: UserModel) -> Self {
        Self {
            id: user.id,
            name: user.name,
            username: user.username,
            password: user.password,
            role_id: user.role_id,
            identity_code: user.identity_code,
            phone: user.phone,
            email: user.email,
            sex: user.sex,
            avatar: user.avatar,
            status: user.status,
            remark: user.remark,
            create_by: user.create_by,
            created_at: user.created_at,
            update_by: user.update_by,
            updated_at: user.updated_at,
            deleted_at: user.deleted_at,
        }
    }
}

pub struct UserDomainCacheRepositoryImpl {}

#[async_trait]
impl CacheRepositoryTrait for UserDomainCacheRepositoryImpl {
    async fn set_captcha(
        &self,
        key: String,
        value: CaptchaCacheInfo,
    ) -> Result<bool, UserDomainError> {
        CacheManager::instance()
            .set_value_ex(&key, &value, 300)
            .await
            .map_err(|e| UserDomainError::CaptchaError(e.to_string()))
    }

    async fn get_captcha(&self, key: String) -> Result<CaptchaCacheInfo, UserDomainError> {
        web_info!("get_captcha:{}", key);
        CacheManager::instance()
            .get_oneuse_value::<CaptchaCacheInfo>(&key)
            .await
            .map_err(|e| match e {
                AppError::CacheNotFoundError(_) => {
                    web_error!("get_captcha err:{:?}", e);
                    UserDomainError::CaptchaNotFound
                }
                _ => UserDomainError::CaptchaError(e.to_string()),
            })
    }
}

pub fn new_user_domain_service() -> UserDomainImpl {
    new_user_domain(
        Box::new(UserDomainCacheRepositoryImpl {}),
        Box::new(UserDomainRepositoryImpl {}),
        Box::new(PwdEncryptImpl {}),
    )
}
