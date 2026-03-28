use crate::prelude::*;


//Структура для сериализации данных из POST-запроса:
#[derive(Serialize, Debug, Deserialize)]
pub struct ReqwestData{
      id: i64,
      article_number: String,
      title_product: String,
      category_id: Option<i64>,
      quantity: i64,
      price: f64
}


#[derive(Serialize)]
pub struct UpdateResult{
    success: bool,
    message: String,
}


pub struct UpdateProduct;

impl UpdateProduct {
    pub async fn update(State(state): State<AppState>, Json(data): Json<ReqwestData>) -> Result<Json<UpdateResult>, AppErrors> {
        let pool = state.pool;
            
        update_in_the_database(pool, data).await?;

        let result = UpdateResult {
            success: true,
            message: "Успешно!".to_string()
        };
        Ok(Json(result))
    }
}

async fn update_in_the_database(pool: Arc<PgPool>, product: ReqwestData) -> Result<(), AppErrors> {
    let result = sqlx::query("UPDATE products SET category_id = $1, quantity = $2, article = $3, name = $4, price = $5 WHERE id = $6")
        .bind(product.category_id)
        .bind(product.quantity)
        .bind(product.article_number)
        .bind(product.title_product)
        .bind(product.price)
        .bind(product.id)
        .execute(&*pool)
        .await?;
        
    if result.rows_affected() < 1 {
        return Err(AppErrors::UpdateError);
    }else{
        return Ok(())
    }
}
