use axum::response::IntoResponse;
use commonx::error::AppError;
use infrastructurex::container::sys_domain::SysDomainRepositoryImpl;

use crate::{controller::SYS_CONTROLLER, resp::ApiResponse};

pub async fn init_all() -> impl IntoResponse {
    ApiResponse::from_result(SYS_CONTROLLER.init_all().await)
}

pub trait SysControllerTrait {
    async fn init_all(&self) -> Result<(), AppError>;
}

pub struct SysController {
    sys_domain_repository_impl: SysDomainRepositoryImpl,
}

impl SysController {
    pub fn new(sys_domain_repository_impl: SysDomainRepositoryImpl) -> Self {
        Self {
            sys_domain_repository_impl,
        }
    }
}

impl SysControllerTrait for SysController {
    async fn init_all(&self) -> Result<(), AppError> {
        self.sys_domain_repository_impl
            .init_all()
            .await
            .map_err(|e| e.into())
    }
}
