use thiserror::Error;
use axum::{
    extract::Json, http::StatusCode, response::{IntoResponse, Redirect}
};
use minijinja::Error as MinijinjaError;
use sqlx::migrate::MigrateError;
use serde_json::json;
use log::error;
use validator::ValidationErrors;



#[derive(Error, Debug)]
pub enum AppErrors {
    #[error("Ошибка! Шаблон не найден!")]
    MinijinjaTemplateError(#[from] MinijinjaError),
    #[error("Ошибка! Сервис временно не работает!")]
    TokioRuntimeError(#[from] std::io::Error),
    #[error("Ошибка! Сервис временно не работает!")]
    SessionError(#[from] tower_sessions::session::Error),
    #[error("Ошибка! Сервис временно не работает!")]
    MigrationsError(#[from] MigrateError),
    #[error("Ошибка! Админ не авторизован!")]
    Unauthorized,
    #[error("Ошибка! Вход не выполнен!")]
    LoginError(String), 
    #[error("Ошибка! Сервис временно не работает!")]
    DeserializeError(#[from] axum::extract::rejection::JsonRejection),
    #[error("Ошибка! Данные не валидны!")]
    ValidationError(ValidationErrors),
    #[error("Ошибка! Сервис временно не работает!")]
    DataBaseError(#[from] sqlx::error::Error),
    #[error("Введен неверный логин или пароль!")]
    ArgonPasswordVerifyError(#[from] argon2::password_hash::Error),
    #[error("Ошибка редактирования! Попробуйте позже!")]
    UpdateError,
    #[error("Ошибка! Удаление пошло не по плану! Попробуйте позже!")]
    NotDeleteError,
    #[error("Ошибка вставки в базу данных")]
    ParseToIntError(#[from] std::num::ParseIntError)
}

impl IntoResponse for AppErrors {
    fn into_response(self) -> axum::response::Response {
       let (status, error_message) = match &self {
            AppErrors::MinijinjaTemplateError(e) => {
              error!("{:?}", e);
              (StatusCode::NOT_FOUND, self.to_string())
            }
            AppErrors::TokioRuntimeError(e) => {
              error!("{:?}", e);
              (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppErrors::SessionError(e) => {
              error!("{:?}", e);
              (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }

            AppErrors::MigrationsError(e) => {
              error!("{:?}", e);
              (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppErrors::Unauthorized => return Redirect::permanent("/login").into_response(),
            AppErrors::LoginError(e) => {
              (StatusCode::UNAUTHORIZED, e.to_string())
            }
            AppErrors::DeserializeError(e) => {
              error!("{:?}", e);
              (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppErrors::ValidationError(_) => {
              (StatusCode::BAD_REQUEST, self.to_string())
            }
            AppErrors::DataBaseError(e) => {
              error!("{:?}", e);
              (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppErrors::ArgonPasswordVerifyError(_) => {
              (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            AppErrors::UpdateError => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()) 
            }
            AppErrors::NotDeleteError => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()) 
            }
            AppErrors::ParseToIntError(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()) 
            }
            
       };

       let body = Json(json!({
            "status": status.as_u16(),
            "error": error_message
       }));
       
       (status, body).into_response()
    }

}
