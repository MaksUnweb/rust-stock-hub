use crate::prelude::*;


//Struct for deserialize data about products that 
//come from the client's side.
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ProductDeserialize {
   pub article_number: String,
    pub title_product: String,
    pub category_product: String,
    pub quantity_product: i32,
    pub price_product: f64 
}

#[derive(Serialize)]
pub struct Product {
   pub article_number: String,
    pub title_product: String,
    pub category_product: Option<i64>,
    pub quantity_product: i32,
    pub price_product: f64 
}


#[derive(Serialize)]
pub struct ResponseResult {
    pub success: bool,
    pub message: String
}


//Struct for entry point in script for 
//processing a request to add new products
pub struct AddFormProducts;


impl AddFormProducts {
    //Method for getting data from the client
    pub async fn add(State(state): State<AppState>, Json(product_deserialize): Json<ProductDeserialize>) -> Result<Json<ResponseResult>, AppErrors> {
        let pool = state.pool;
   
        let product: Product = if product_deserialize.category_product == "null" || product_deserialize.category_product.is_empty() {
            Product { 
                article_number: product_deserialize.article_number, 
                title_product: product_deserialize.title_product, 
                category_product: None, 
                quantity_product: product_deserialize.quantity_product, 
                price_product: product_deserialize.price_product
            }
        }else {
            Product { 
                article_number: product_deserialize.article_number, 
                title_product: product_deserialize.title_product, 
                category_product: Some(product_deserialize.category_product.parse::<i64>().unwrap()), 
                quantity_product: product_deserialize.quantity_product, 
                price_product: product_deserialize.price_product
            }
        };
        
        insert_into_db(product, pool).await?;
        
       let response_true = ResponseResult {
            success: true,
            message: "Успешно добавлено!".to_string()
       };

       Ok(Json(response_true))
    }

}

#[allow(dead_code)]
async fn insert_into_db(product: Product, pool: Arc<PgPool>) -> Result<(), AppErrors> {
    sqlx::query("INSERT INTO products (category_id, quantity, article, name, price) VALUES ($1, $2, $3, $4, $5)")
        .bind(product.category_product)
        .bind(product.quantity_product)
        .bind(product.article_number)
        .bind(product.title_product)
        .bind(product.price_product)
        .execute(&*pool)
        .await?;

    Ok(())
}


