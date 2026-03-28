use sqlx::{
    PgPool, 
    postgres::PgPoolOptions
};
use log::error;

//The struct for creating a database connection
pub struct DataBase;

impl DataBase {
   pub async fn connect(max_connections: u32, url: String) -> PgPool  {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(&url).await.map_err(|e| {
              error!("Error creating a database connection: {e}");
            }).expect("Error creating a database connection");

        pool
    }
}
