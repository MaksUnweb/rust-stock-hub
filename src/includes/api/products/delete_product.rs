use crate::prelude::*;

pub struct DeleteProduct;


#[derive(Serialize)]
pub struct DeleteResult {
    pub success: bool,
    pub message: String
}

#[derive(Deserialize)]
pub struct ReqwestData {
    id: i64
}

impl DeleteProduct {
   pub async fn delete(State(state): State<AppState>, Json(data): Json<ReqwestData>) -> Result<Json<DeleteResult>, AppErrors> {
       let pool = state.pool;
        let result = DeleteResult {
            success: true,
            message: "Успешно!".to_string()
        };

        delete_for_db(pool, data.id).await?;

        Ok(Json(result))
   } 
}


async fn delete_for_db(pool: Arc<PgPool>, id: i64) -> Result<(), AppErrors> {
    let result = sqlx::query("DELETE FROM products WHERE id = $1")
        .bind(id)
        .execute(&*pool)
        .await?;

    if result.rows_affected() == 0  {
        return Err(AppErrors::NotDeleteError);
    }else{
        return Ok(())
    }
}
