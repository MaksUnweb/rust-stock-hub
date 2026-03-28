use crate::prelude::*;
use axum::extract::Query;
use crate::includes::api::response::Response;


pub struct SelectProducts;

//Структура для получения данных с запроса:
#[derive(Serialize, Deserialize)]
pub struct ReqwestData {
    pub id: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>
}


//Структура для хранения данных о продукте:
#[derive(Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: i64,
    pub category_id: Option<i64>,
    pub quantity: i32,
    pub article: String,
    pub name: String,
    pub price: f64
}


impl SelectProducts {
   pub async fn select(Query(data): Query<ReqwestData>, State(state): State<AppState>) -> Result<Json<Response<Product>>, AppErrors> {
       let pool = state.pool;
        if data.limit.is_some() && data.offset.is_some() {
           let products = select_with_pagination(pool, data.limit, data.offset).await?;

           return Ok(Json(Response::ok(products, "Успешно!".to_string())))
        }    

        if data.id.is_some() {
            let product = select_with_id(pool, data.id).await?;
           return Ok(Json(Response::ok(product, "Успешно!".to_string())))
        }

        if data.limit.is_some() && data.offset.is_none() && data.id.is_none() {
            let products = select_past_products(pool, data.limit).await?;
            return Ok(Json(Response::ok(products, "Успешно!".to_string())))
        }

        Ok(Json(Response::err("Сервис временно недоступен! Попробуйте позже!".to_string())))
   } 
}


//Функция для вывода всех продуктов с пагинацией:
async fn select_with_pagination(pool: Arc<PgPool>, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Product>, AppErrors> {
    let limit = limit.unwrap();
    let offset = offset.unwrap();


    let products: Vec<Product> = sqlx::query_as::<_, Product>("SELECT id, category_id, quantity, article, name, price::DOUBLE PRECISION FROM products ORDER BY id DESC LIMIT $1 OFFSET $2")
        .bind(limit)
        .bind(offset)
        .fetch_all(&*pool) 
        .await?;

    Ok(products)
}

//Метод для вывода продукта по id:
async fn select_with_id(pool: Arc<PgPool>, id: Option<i64>) -> Result<Vec<Product>, AppErrors> {
    let mut vec_product: Vec<Product> = Vec::new();
    let id = id.unwrap();

    let product: Product = sqlx::query_as::<_, Product>("SELECT id, category_id, quantity, article, name, price::DOUBLE PRECISION FROM products WHERE id = $1")
        .bind(id)
        .fetch_one(&*pool)
        .await?;
    vec_product.push(product);
    Ok(vec_product)
}

//Функция для вывода последних 5 продуктов:
async fn select_past_products(pool: Arc<PgPool>, limit: Option<i64>) -> Result<Vec<Product>, AppErrors> {
    let limit = limit.unwrap();

    let products: Vec<Product> = sqlx::query_as::<_, Product>("SELECT id, category_id, quantity, article, name, price::DOUBLE PRECISION FROM products ORDER BY id DESC LIMIT $1")
        .bind(limit)
        .fetch_all(&*pool) 
        .await?;

    Ok(products)
}

