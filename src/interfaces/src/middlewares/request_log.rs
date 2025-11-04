use axum::{body::Body, extract::Request, middleware::Next, response::IntoResponse};
use hyper::StatusCode;
use infrastructurex::web_info;

use crate::middlewares::{ReqCtx, parse_ip};

pub async fn request_log_fn_mid(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // 获取客户端IP地址
    let ip = parse_ip(&req);
    let (parts, body) = req.into_parts();

    let method = parts.method.to_string();
    let uri = &parts.uri;
    // let path = uri.path();
    let query = uri.query().unwrap_or("");

    let user_agent = parts
        .headers
        .get("user-agent")
        .map_or("", |h| h.to_str().unwrap_or(""));

    // let content_type = parts
    //     .headers
    //     .get(CONTENT_TYPE)
    //     .map_or("", |h| h.to_str().unwrap_or(""));

    // 读取请求体
    let body_bytes = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("读取请求体失败: {}", e)))?;

    let body_content = String::from_utf8_lossy(&body_bytes);

    // 记录日志
    web_info!(
        "ip:{} method:{} url:{} query:{} body:{}",
        ip,
        method,
        uri,
        query,
        if body_content.len() > 500 {
            format!("{}...(truncated)", &body_content[..500])
        } else {
            body_content.to_string()
        }
    );
    let req_ctx = ReqCtx {
        ip: ip,
        ori_uri: uri.to_string(),
        path: uri.path().to_string(),
        path_params: uri.path().to_string(),
        method: method,
        // user_agent: user_agent.to_string(),
    };

    // 重新构建请求
    let mut rebuilt_request = Request::from_parts(parts, Body::from(body_bytes));
    rebuilt_request.extensions_mut().insert(req_ctx);
    let res_end = next.run(rebuilt_request).await;
    Ok(res_end)
}
