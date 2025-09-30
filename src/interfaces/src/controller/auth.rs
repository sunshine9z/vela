// pub async fn gen_captcha(arg: ClientInfo) -> CaptchaImage {}

use axum::response::IntoResponse;
use infrastructurex::web_info;

use crate::resp::ApiResponse;

pub async fn gen_captcha() -> impl IntoResponse {
    web_info!("gen captcha");
    ApiResponse::ok()
}
