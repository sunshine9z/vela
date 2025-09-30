use axum::response::{IntoResponse, Response};
use commonx::{error::AppError, traits::IntoStatusTuple};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct RespDataString(pub String);

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub data: T,
    pub message: String,
}

impl<T> ApiResponse<T>
where
    T: Serialize,
{
    pub fn new(code: StatusCode, data: T, message: String) -> Self {
        Self {
            code: code.as_u16(),
            data,
            message,
        }
    }
    pub fn ok_with_data(data: T) -> Response {
        (
            StatusCode::OK,
            Self::new(StatusCode::OK, data, "操作成功".to_string()),
        )
            .into_response()
    }

    pub fn ok_with_data_and_msg(data: T, msg: impl Into<String>) -> Response {
        (StatusCode::OK, Self::new(StatusCode::OK, data, msg.into())).into_response()
    }

    pub fn from_result(result: Result<T, AppError>) -> Response
    where
        T: Serialize,
    {
        match result {
            Ok(data) => Self::ok_with_data(data),
            Err(err) => Self::from_error(err),
        }
    }
    pub fn from_error(err: AppError) -> Response {
        let (status, message) = err.into_status_tuple();
        ApiResponse::error_response(status, message)
    }
}

impl ApiResponse<()> {
    pub fn from_empty_result(result: Result<(), AppError>) -> Response {
        match result {
            Ok(()) => Self::ok(),
            Err(err) => Self::from_error(err),
        }
    }
    // 统一的错误响应方法 - 返回 ApiResponse<EmptyData>
    pub fn error_response(status: StatusCode, message: String) -> Response {
        (status, ApiResponse::new(status, (), message)).into_response()
    }
    pub fn not_found(msg: impl Into<String>) -> Response {
        ApiResponse::error_response(StatusCode::NOT_FOUND, msg.into())
    }

    pub fn ok() -> Response {
        (
            StatusCode::OK,
            ApiResponse::new(StatusCode::OK, (), "操作成功".to_string()),
        )
            .into_response()
    }

    pub fn bad_request(msg: impl Into<String>) -> Response {
        ApiResponse::error_response(StatusCode::BAD_REQUEST, msg.into())
    }
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match serde_json::to_string(&self) {
            Ok(json) => (
                [
                    ("Content-Type", "application/json;charset=UTF-8"),
                    ("Access-Control-Allow-Origin", "*"),
                    ("Cache-Control", "no-cache"),
                ],
                json,
            )
                .into_response(),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                [("Content-Type", "application/json;charset=UTF-8")],
                r#"{"message":"序列化响应失败","data":{}}"#,
            )
                .into_response(),
        }
    }
}

#[allow(unused_macros)]
macro_rules! res_ok {
    () => {
        ApiResponse::ok()
    };
    ($data:expr) => {
        ApiResponse::ok_with_data($data)
    };
    ($data:expr, $msg:expr) => {
        ApiResponse::ok_with_data_and_msg($data, $msg)
    };
}

#[allow(unused_macros)]
macro_rules! res_bad_request {
    ($msg:expr) => {
        ApiResponse::bad_request($msg)
    };
}
