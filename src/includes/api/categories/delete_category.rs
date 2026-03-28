use crate::prelude::*;


//Структура для возвращения ответа:
#[derive(Serialize)]
pub struct DeleteResult{
    success: bool,
    message: String,
}

#[derive(Deserialize, Debug)]
pub struct ReqwestData {
    id: i64
}


pub struct DeleteCategory;


impl DeleteCategory {
    pub async fn delete(State(state): State<AppState>, Json(data): Json<ReqwestData>) -> Result<Json<DeleteResult>, AppErrors>  {
        let pool = state.pool;
        let result = DeleteResult {
            success: true,
            message: "Успешно!".to_string()
        };

        delete_from_db(pool, data.id).await?;

        Ok(Json(result))
    }
}

async fn delete_from_db(pool: Arc<PgPool>, id: i64) -> Result<(), AppErrors>{

    let result = sqlx::query("DELETE FROM categories WHERE id = $1")
        .bind(id)
        .execute(&*pool)
        .await?;

    if result.rows_affected() < 1 {
        return Err(AppErrors::NotDeleteError)
    }else {
        return Ok(())
    }
}

