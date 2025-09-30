use axum::{extract::Request, middleware::Next, response::Response};
use hyper::StatusCode;

pub async fn check_permission_mid(
    req: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    // let ctx = req.extensions().get::<ReqCtx>().expect("ReqCtx not found");
    // let user = req.extensions().get::<UserInfo>().expect("用户信息不存在");

    Ok(next.run(req).await)
}
