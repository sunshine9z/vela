use std::time::Instant;

use axum::{extract::Request, middleware::Next, response::IntoResponse};
use chrono::Local;
use hyper::StatusCode;
use infrastructurex::container::operater_log_domain::new_operater_log_domain_service;
use once_cell::sync::Lazy;
use operaterLogDomain::{
    OperaterLogDomainImpl, api::traits::OperaterLogDomainTrait, entity::OperaterLog,
};

use crate::{middlewares::ReqCtx, resp::RespDataString, types::user_info::UserInfo};

pub static OPERATOR_LOG_DOMAIN: Lazy<OperaterLogDomainImpl> =
    Lazy::new(|| new_operater_log_domain_service());

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
    let _ = OPERATOR_LOG_DOMAIN
        .create(OperaterLog {
            api_name: req_ctx.path.clone(),
            oper_ip: req_ctx.ip.clone(),
            oper_id: user_ctx.id,
            oper_name: user_ctx.username.clone(),
            oper_url: req_ctx.ori_uri.clone(),
            oper_location: req_ctx.ori_uri.clone(),
            request_method: req_ctx.method.clone(),
            oper_param: req_ctx.path_params.clone(),
            json_result: if respdata.len() > 1024 {
                respdata.chars().take(1024).collect::<String>()
            } else {
                respdata.clone()
            },
            cost_time: duration.as_millis() as i64,
            oper_time: Local::now(),
            ..Default::default()
        })
        .await;
}
