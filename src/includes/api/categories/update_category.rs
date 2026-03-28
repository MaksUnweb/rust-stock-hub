use crate::prelude::*;


#[derive(Deserialize)]
pub struct ReqwestData {
      id: i64,
      name: String,
      parent_id: Option<i64>
}

#[derive(Serialize)]
pub struct UpdateResult {
    success: bool,
    message: String
}



pub struct UpdateCategory;

impl UpdateCategory {
    pub async fn update(State(state): State<AppState>, Json(data): Json<ReqwestData>) -> Result<Json<UpdateResult>, AppErrors> {
        let pool = state.pool; 

        update_from_db(pool, data.parent_id, data.name, data.id).await?;

        let result = UpdateResult {
            success: true,
            message: "Успешно!".to_string()
        };

        Ok(Json(result))
    }
}


async fn update_from_db(pool: Arc<PgPool>, parent_id: Option<i64>, name: String, category_id: i64) -> Result<(), AppErrors> {
    let result = sqlx::query("UPDATE categories SET parent_id = $1, name = $2 WHERE id = $3")
        .bind(parent_id)
        .bind(name)
        .bind(category_id)
        .execute(&*pool)
        .await?;

    if result.rows_affected() > 0 {
        return Ok(())
    }else{
        return Err(AppErrors::UpdateError);
    }
}
