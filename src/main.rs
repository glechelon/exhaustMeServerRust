mod controllers;
mod domain;

use mysql::*;
use mysql::prelude::*;

const URL_DB : &str = "mysql://guillaume:enzo1542@localhost:3306/EXHAUST_ME";

use actix_web::{App, HttpServer};

 fn main() {
    let pool = connect_db();
    if pool.is_ok() {
        let mut opt_connexion = pool.unwrap().get_conn();
        if opt_connexion.is_ok() {
            let mut connexion = opt_connexion.unwrap();
            connexion.query_drop(
            r"CREATE TEMPORARY TABLE payment (
                customer_id int not null,
                amount int not null,
                account_name text
            )").expect("erreur");
        connexion.query_drop("SELECT COUNT(*) FROM payment");
        }
    }

    

    serve();

}


#[actix_rt::main]
async fn serve() ->std::io::Result<()>{
    HttpServer::new(|| {
        App::new().service(controllers::user_controller::get_users)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await

}


fn connect_db() -> Result<Pool>{
   return  Pool::new(URL_DB);
}
