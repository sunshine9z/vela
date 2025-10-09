// pub async fn gen_captcha(arg: ClientInfo) -> CaptchaImage {}

use axum::response::IntoResponse;
use userDomain::entity::captcha::CaptchaImage;

use crate::resp::ApiResponse;

pub async fn gen_captcha() -> impl IntoResponse {
    // CaptchaImage {
    //     image: "".to_string(),
    //     code: "".to_string(),
    // }
    ApiResponse::ok_with_data(CaptchaImage {
        image: "".to_string(),
        code: "".to_string(),
    })
}
