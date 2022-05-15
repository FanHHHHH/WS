use actix_web::{error, http::StatusCode, web, HttpResponse, Result};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum MyError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    err_message: web::Json<String>,
}

impl MyError {
    fn error_response(&self) -> web::Json<String> {
        match self {
            MyError::DBError(msg) => {
                println!("数据库出现错误：{}", msg);
                web::Json("数据库错误！".to_string())
            }
            MyError::ActixError(msg) => {
                println!("服务器内部错误：{}", msg);
                web::Json("服务器内部错误！".to_string())
            }
            MyError::NotFound(msg) => {
                println!("服务未找到：{}", msg);
                web::Json(msg.to_string())
            }
            MyError::InvalidInput(msg) => {
                println!("非法输入：{}", msg);
                web::Json("非法输入！".to_string())
            }
        }
    }
}

impl error::ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::DBError(_msg) | MyError::ActixError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_msg) => StatusCode::NOT_FOUND,
            MyError::InvalidInput(_msg) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            err_message: self.error_response(),
        })
    }
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for MyError {
    fn from(err: actix_web::error::Error) -> Self {
        MyError::ActixError(err.to_string())
    }
}

impl From<SQLxError> for MyError {
    fn from(err: SQLxError) -> Self {
        MyError::ActixError(err.to_string())
    }
}
