use axum::{
    Router, extract::{Path, State}, 
    response::Html, 
    routing::{get, post}
};
use minijinja::{Environment, context};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
use std::sync::Arc;
use tower_sessions_sqlx_store::{sqlx::PgPool, PostgresStore};
use tower_sessions::{SessionManagerLayer, Expiry, Session};
use time::Duration;
use log::error;
use crate::prelude::*;

//-------------------Add needs crates:-------------------

//Crate for connect database:
use crate::includes::db::DataBase;
use crate::includes::api::products::add_form_products::AddFormProducts;
use crate::includes::api::login::Login;
use crate::includes::api::categories::add_category::AddCategory;
use crate::includes::api::products::delete_product::DeleteProduct;
use crate::includes::api::products::update_product::UpdateProduct;
use crate::includes::api::categories::delete_category::DeleteCategory;
use crate::includes::api::categories::update_category::UpdateCategory;
use crate::includes::api::products::search::Search;
use crate::includes::api::categories::select_categories::SelectCategory;
use crate::includes::api::products::select_products::SelectProducts;

//-------------------------------------------------------

type ServerResult = Result<(), AppErrors>;


const SESSION_KEY: &str = "admin_session";

//Structure for the global application state:
#[derive(Clone)]
pub struct AppState {
    pub template: Arc<Environment<'static>>,
    pub pool: Arc<PgPool>
}



//Trait for critical errors:
trait CriticalErrors<T> {
   fn log_error(self, msg: &str) -> T;
}

//Implementation of a trait with critical errors:
impl<T, E: std::fmt::Display> CriticalErrors<T> for Result<T, E> {
    fn log_error(self, msg: &str) -> T {
       match self {
            Ok(val) => val,
            Err(e) => {
                error!("{}: {}", msg, e);
                panic!("{}: {}", msg, e);
            }
       } 
    }
}

pub struct Web;

#[derive( Deserialize, Serialize)]
struct SessionData(usize);

impl Web {

    //A public method for creating routes and starting the server
    pub async fn start_server(&self) -> ServerResult {

        //Making env for  templates storage:
        let mut env = Environment::new();
        env.add_template("main", include_str!("../templates/main.html"))?;
        env.add_template("login", include_str!("../templates/login.html"))?;
        env.add_template("header", include_str!("../templates/header.html"))?;
        env.add_template("footer", include_str!("../templates/footer.html"))?;
        env.add_template("base-main", include_str!("../templates/base_main.html"))?;
        env.add_template("new-product", include_str!("../templates/new_product.html"))?;
        env.add_template("new-category", include_str!("../templates/new_category.html"))?;
        env.add_template("error-template", include_str!("../templates/404.html"))?;



        //Creating a database connection:
        let pool = DataBase::connect(5, "postgres://Admin:1111@localhost/storage_db".to_string()).await;
        let cloned_pool = pool.clone();
        
        

        //Creating storage for sessions in Database:
        let session_store = PostgresStore::new(cloned_pool)
            .with_schema_name("public").log_error("Error adding sessions to the public schema")
            .with_table_name("sessions").log_error("Error in setting the session table name");

        //session_store.migrate().await.log_error("Error creating a table with a session");
        session_store.migrate().await.log_error("Error init migrations!");

        //Create migrations:
        sqlx::migrate!("./migrations").run(&pool).await?;

        let session_layer = SessionManagerLayer::new(session_store)
            .with_secure(false)
            .with_same_site(tower_sessions::cookie::SameSite::Lax)
            .with_http_only(false)
            .with_expiry(Expiry::OnInactivity(Duration::hours(1)));


        //Making state app: 
        let state = AppState { template: Arc::new(env), pool: Arc::new(pool) };
        
        let app = Router::new() 
            .route("/login", get(login_handler))
            .route("/", get(main_handler))
            .route("/{page_name}", get(other_templates_handler))
            .route("/api/products/add_product", post(AddFormProducts::add))
            .route("/api/products/select_products", get(SelectProducts::select))
            .route("/api/products/delete_product", post(DeleteProduct::delete))
            .route("/api/products/update_product", post(UpdateProduct::update))
            .route("/api/products/search", post(Search::start))
            .route("/api/categories/delete_category", post(DeleteCategory::delete))
            .route("/api/categories/update_category", post(UpdateCategory::update))
            .route("/api/categories/add_category", post(AddCategory::add))
            .route("/api/category/select_categories", get(SelectCategory::select))
            .route("/api/login", post(Login::start_login))
            .fallback(error_template_handler)
            .with_state(state)
            .nest_service("/static", ServeDir::new("static"))
            .layer(session_layer);

        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
        axum::serve(listener, app.into_make_service()).await?;

            Ok(())
    }
}

//Function for rendering the main page template:
async fn main_handler(session: Session, State(state): State<AppState>) -> Result<Html<String>, AppErrors> {

    //Check session:
    let _is_auth = match check_session(session).await {
        Ok(_) => {}
        Err(_) => {
            return Err(AppErrors::Unauthorized);
        }
    };


    let template_main = state.template.get_template("main")?;
    let load_template = state.template.get_template("base-main")?;
    let inner_html = load_template.render(context! {})?;
    let html = template_main.render(context! {
        load_data => &inner_html
    })?;
    Ok(Html(html))
}

async fn other_templates_handler(session: Session, State(state): State<AppState>, Path(page_name): Path<String>) -> Result<Html<String>, AppErrors> {

    //Check session:
    let _is_auth = match check_session(session).await {
        Ok(_) => {}
        Err(_) => {
            return Err(AppErrors::Unauthorized);
        }
    };


    let template_main = state.template.get_template("main")?;
    let load_template = state.template.get_template(&page_name)?;
    let inner_html = load_template.render(context! {})?;
    let html = template_main.render(context! {
        load_data => &inner_html
    })?;
    Ok(Html(html))
}

async fn login_handler(State(state): State<AppState>) -> Result<Html<String>, AppErrors> {
    let template = state.template.get_template("login")?;
    let html = template.render(context! {
        //Data for the template:
    })?;
    Ok(Html(html))
}

async fn error_template_handler(State(state): State<AppState>) -> Result<Html<String>, AppErrors> {
    let template = state.template.get_template("error-template")?;
    let html = template.render(context! {
        //Data for the template 
    })?;
    Ok(Html(html))
}

async fn check_session(session: Session) -> Result<(), AppErrors> {
    let session_data: Option<SessionData> = session.get(SESSION_KEY).await.unwrap();
    match session_data {
        Some(_) => {
            Ok(())
        }
        None => {
            Err(AppErrors::Unauthorized)
        }
    }
}

