use crate::prelude::*;
use axum::extract::Query;


pub struct SelectCategory;


//Структура для получения данных с запроса:
#[derive(Serialize, Deserialize)]
pub struct ReqwestData {
    pub id: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>
}

#[derive(Deserialize, Serialize, FromRow)]
pub struct Category {
    pub id: i64,
    pub parent_id:Option<i64>,
    pub name: String
}

impl SelectCategory {
    //Проверяем, какой вариант вывода необходим, запускаем вывод, возвращаем ответ
    pub async fn select(Query(req_data): Query<ReqwestData>, State(state): State<AppState>) -> Result<Json<Response<Category>>, AppErrors> {
        let pool = state.pool;

           if req_data.limit.is_some() && req_data.offset.is_some() {
                let categories = pagination_select(pool ,req_data.limit, req_data.offset).await?;
                return Ok(Json(Response::ok(categories, "Успешно!".to_string())))
           }

           if req_data.id.is_some() {
                let category = select_where_id(pool, req_data.id).await?;
                let mut vec_cat = Vec::new();
                vec_cat.push(category);
                return Ok(Json(Response::ok(vec_cat, "Успешно!".to_string())))
           }

           if req_data.id.is_none() && req_data.limit.is_none() && req_data.offset.is_none() {
                let categories = select_all_category(pool).await?;
                return Ok(Json(Response::ok(categories, "Успешно!".to_string())))
           }

           if req_data.limit.is_some() && req_data.offset.is_none() && req_data.id.is_none() {
                let categories = select_past_categories(pool, req_data.limit).await?;
                return Ok(Json(Response::ok(categories, "Успешно!".to_string())))
           }

           Ok(Json(Response::err("Ошибка!".to_string())))
    }
}


//Функция для вывода категорий с пагинацией:
async fn pagination_select(pool: Arc<PgPool>, limit: Option<i64>, offset: Option<i64>) -> Result<Vec<Category>, AppErrors> {
    let limit = limit.unwrap();
    let offset = offset.unwrap();

    let categories: Vec<Category> = sqlx::query_as::<_, Category>("SELECT id, parent_id, name FROM categories ORDER BY id DESC LIMIT $1 OFFSET $2")
        .bind(limit)
        .bind(offset)
        .fetch_all(&*pool)
        .await?;

    Ok(categories)
}


//Функция для вывода одной категории по id:
async fn select_where_id(pool: Arc<PgPool>, id: Option<i64>) -> Result<Category, AppErrors> {
    let category: Category = sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = $1")
        .bind(id)
        .fetch_one(&*pool) 
        .await?;
    Ok(category)
}

//Функция для вывода всех категорий (без параметров):
async fn select_all_category(pool: Arc<PgPool>) -> Result<Vec<Category>, AppErrors> {
    let categories: Vec<Category> = sqlx::query_as::<_, Category>("SELECT * FROM categories")
        .fetch_all(&*pool)
        .await?;

    Ok(categories)
}


//Вывод последних 5 категорий:
async fn select_past_categories(pool: Arc<PgPool>, limit: Option<i64>) -> Result<Vec<Category>, AppErrors> {
    let limit = limit.unwrap();
    let categories: Vec<Category> = sqlx::query_as::<_, Category>("SELECT * FROM categories ORDER BY id DESC LIMIT $1")
        .bind(limit)
        .fetch_all(&*pool)
        .await?;

    Ok(categories)
}

