//Here is the implementation of adding categories to the database
use crate::prelude::*;


//Structure for serializing incoming data
#[derive(Deserialize, FromRow, Debug)]
pub struct CategoryDeserialize {
    pub name: String, 
    pub parent_id: String
}

struct Category {
    name: String,
    parent_id: Option<i64>
}


//The structure for returning the response
#[derive(Serialize)]
pub struct CategoryResult {
    pub success: bool,
    pub message: String
}

pub struct AddCategory;


impl AddCategory {
    pub async fn add(State(state): State<AppState>, Json(category_deserialize): Json<CategoryDeserialize>) -> Result<Json<CategoryResult>, AppErrors> {
        let pool = state.pool;

        //Processing the structure. Since there is not always a parent category
        let category: Category = if category_deserialize.parent_id == "null" || category_deserialize.parent_id.is_empty() {
            Category {
                name: category_deserialize.name,
                parent_id: None
            }
        } else {
            Category {
                name: category_deserialize.name,
                parent_id: Some(category_deserialize.parent_id.parse::<i64>()?)
            }
        };

        //Insert inro database:
        insert_into_db(category, pool).await?;


        let result = CategoryResult {
            success: true,
            message: "Успешно!".to_string()
        };

        Ok(Json(result))
    }
}

async fn insert_into_db(category: Category, pool: Arc<PgPool>) -> Result<(), AppErrors> {
    
    sqlx::query("INSERT INTO categories (parent_id, name) VALUES ($1, $2)")
        .bind(category.parent_id)
        .bind(category.name)
        .execute(&*pool)
        .await?;


    Ok(())
}


