mod includes;
use env_logger;
use crate::includes::start_web::Web;
use log::error;
pub mod prelude;

#[tokio::main]
async fn main() {

   env_logger::init(); 

   let web = Web;
    
   match web.start_server().await{
        Ok(_) => {},
        Err(e) => {
            error!("{:?}", e);
        }
   };

}
