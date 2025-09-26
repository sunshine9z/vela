use axum::{
    body::Body,
    extract::{OriginalUri, Request},
    middleware::Next,
    response::{IntoResponse, Response},
};
use http_body_util::BodyExt;
use hyper::StatusCode;

use crate::{
    API_PATH_PRE,
    middlewares::{ReqCtx, parse_ip},
};

pub async fn req_info_fn_mid(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let ori_uri_path = if let Some(path) = req.extensions().get::<OriginalUri>() {
        path.0.path().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let path = ori_uri_path.replacen(API_PATH_PRE, "", 1);
    let method = req.method().to_string();
    let path_params = req.uri().query().unwrap_or("").to_string();

    // 获取客户端IP地址
    let ip = parse_ip(&req);

    let (parts, body) = req.into_parts();

    let bytes = body
        .collect()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())
        .unwrap()
        .to_bytes();

    let req_ctx: ReqCtx = ReqCtx {
        ip,
        ori_uri: if path_params.is_empty() {
            ori_uri_path
        } else {
            format!("{}?{}", ori_uri_path, path_params)
        },
        path,
        path_params,
        method: method.to_string(),
    };
    let mut req = Request::from_parts(parts, Body::from(bytes));
    req.extensions_mut().insert(req_ctx);
    Ok(next.run(req).await)
}

pub async fn check_permission_mid(
    req: Request,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    // let ctx = req.extensions().get::<ReqCtx>().expect("ReqCtx not found");
    // let user = req.extensions().get::<UserInfo>().expect("用户信息不存在");

    Ok(next.run(req).await)
}
