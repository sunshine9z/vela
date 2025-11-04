use std::time::Instant;

use axum::{extract::Request, middleware::Next, response::IntoResponse};
use hyper::StatusCode;

use crate::{middlewares::ReqCtx, resp::RespDataString, types::user_info::UserInfo};

pub async fn operate_log_fn_mid(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let req_ctx = match req.extensions().get::<ReqCtx>() {
        Some(ctx) => ctx.clone(),
        None => return Ok(next.run(req).await),
    };

    let user_ctx = match req.extensions().get::<UserInfo>() {
        Some(ctx) => ctx.clone(),
        None => return Ok(next.run(req).await),
    };

    let now = Instant::now();
    let res_end = next.run(req).await;
    let duration = now.elapsed();
    let respdata = match res_end.extensions().get::<RespDataString>() {
        Some(x) => &x.0,
        None => &"".to_string(),
    };
    oper_log_add(&req_ctx, &user_ctx, respdata, duration).await;
    Ok(res_end)
}
pub async fn oper_log_add(
    req_ctx: &ReqCtx,
    user_ctx: &UserInfo,
    respdata: &String,
    duration: std::time::Duration,
) {
    // TODO 写入操作日志
}
