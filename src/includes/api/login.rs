use argon2::{Argon2, PasswordHash, PasswordVerifier};
use std::sync::LazyLock;
use validator::Validate;
use regex::Regex;
use tower_sessions::Session;

use crate::prelude::*;


static RE_LOGIN: LazyLock<Regex> = LazyLock::new(|| {
   Regex::new(r"^[a-zA-Z0-9@_]+$").unwrap()
});

const SESSION_KEY: &str = "admin_session";


#[derive(Debug, Validate, Deserialize)]
pub struct ValidateData {
    #[validate(regex(path = *RE_LOGIN))]
    admin_login: String,
    #[validate(regex(path = *RE_LOGIN))]
    admin_password: String
}


//Эта структура нужна для возвращения результата входа
#[derive(Serialize)]
pub struct LoginResult {
    pub success: bool,
    pub message: String
}

#[derive(Debug, Deserialize)]
pub struct Admin {
    pub admin_login: String,
    pub admin_password: String
}

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
struct DataBaseAdmin{
    id: i64,
    login: String,
    password: String
}


pub struct Login;


impl Login {
   pub async fn start_login(State(state): State<AppState>, session: Session, Json(admin): Json<Admin>) -> Result<Json<LoginResult>, AppErrors> {
        let pool = state.pool;
        let admin_login = admin.admin_login;
        let admin_password = admin.admin_password;

        let cloned_login = admin_login.clone();
        let cloned_password = admin_password.clone();



        let response_true = LoginResult {
            success: true,
            message: "Успешно!".to_string()
        };


        validate_data(admin_login, admin_password).await?;

        //We check the admin in the database and, if there is, we receive his data for processing:
        let admin_db  = check_in_db(&*pool, cloned_login).await?;
        let admin_db = admin_db.ok_or(AppErrors::LoginError("Ошибка! Неверный логин или пароль!".to_string()))?;

        //Verify passsword:
        match password_verify(cloned_password, admin_db.password).await {
            Ok(_) => {

                session.insert(SESSION_KEY, admin_db.id).await.unwrap();
                Ok(Json(response_true))
            }
            Err(e) => Ok(Json(LoginResult { success: false, message: e.to_string() })),
        }
    }
}


//Function for validation:
async fn validate_data(admin_login: String, admin_password: String) -> Result<(), AppErrors>{
    let valid = ValidateData {
        admin_login: admin_login,
        admin_password: admin_password
    };
    //Checking for validity
    valid.validate().map_err(AppErrors::ValidationError)?;
    Ok(())
}

//Function for check admin in db:
async fn check_in_db(pool: &sqlx::PgPool, admin_login: String) -> Result<Option<DataBaseAdmin>, AppErrors> {
    let admin = sqlx::query_as::<_, DataBaseAdmin>("SELECT * FROM admins WHERE login = $1")
        .bind(admin_login)
        .fetch_optional(pool)
        .await?;

    Ok(admin)
}


//Function for password verification:
async fn password_verify(input_password: String, hash: String) -> Result<(), AppErrors> {
    let parsed_hash = PasswordHash::new(&hash)?;
    let argon2 = Argon2::default();

     match argon2.verify_password(input_password.as_bytes(), &parsed_hash) {
        Ok(_) => {
           return Ok(()); 
        }
        Err(e) => {
            return Err(AppErrors::ArgonPasswordVerifyError(e));
        }
    };
}


