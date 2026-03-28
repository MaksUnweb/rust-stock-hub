use crate::prelude::*;


#[derive(Serialize, Deserialize, Debug)]
pub struct ReqwestData {
    pub data: String
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: i64,
    pub category_id: Option<i64>,
    pub quantity: i32,
    pub article: String,
    pub name: String,
    pub price: f64
}

pub struct Search;

impl Search {
    pub async fn start(State(state): State<AppState>, Json(data): Json<ReqwestData>) -> Result<Json<Response<Product>>, AppErrors> {
        let pool = state.pool;
    
        let products = search_from_db(pool, data.data).await?;
        if products.iter().len() > 0 {
            return Ok(Json(Response::ok(products, "Успешно!".to_string())))
        }else{
            return Ok(Json(Response::err("Поиск ничего не обноружил...".to_string())))
        }
    }
}

async fn search_from_db(pool: Arc<PgPool>, data: String) -> Result<Vec<Product>, AppErrors> {
    let products: Vec<Product> = sqlx::query_as::<_, Product>(
        r#"SELECT id, category_id, quantity, article, name, price::DOUBLE PRECISION FROM products WHERE article ILIKE $1 ORDER BY id DESC LIMIT 5"#
        )
        .bind(format!("%{}%", data))
        .fetch_all(&*pool)
        .await?;

    Ok(products)
}
