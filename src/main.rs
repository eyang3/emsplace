mod types;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::env;
use r2d2_postgres::postgres::{NoTls, Client};
use r2d2_postgres::PostgresConnectionManager;


#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref POOL: r2d2::Pool<PostgresConnectionManager<NoTls>> = {
        let manager = PostgresConnectionManager::new(
            "host=localhost user=iamspazzy dbname=emsplace".parse().unwrap(),
            NoTls,
        );
        r2d2::Pool::new(manager).unwrap()
    };
}


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
#[get("/db_info")]
async fn get_data() -> impl Responder {
    
    let mut client = POOL.get().unwrap();
    /* for row in client.query("select * from test_table", &[]).unwrap(){
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        println!("found player: {} {}", id, name);
    } */
    HttpResponse::Ok().body("Hello world!")


}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
    
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .expect("PORT must be a number");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .service(get_data)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0",port))?
    .run()
    .await
}
