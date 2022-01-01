mod types;
mod routes;
mod db;

use actix_web::dev::Service;
use actix_web::http::{header, HeaderValue, HeaderName};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use r2d2_postgres::postgres::{Client, NoTls};
use r2d2_postgres::PostgresConnectionManager;
use std::env;

#[macro_use]
extern crate lazy_static;



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/db_info")]
async fn get_data() -> impl Responder {
    let mut client = db::POOL.get().unwrap();
    let mut v: Vec<types::Person> = Vec::new();
    for row in client.query("select * from test_table", &[]).unwrap() {
        let u = types::Person {
            id: row.get(0),
            data: row.get(1),
        };
        v.push(u);
    }
    HttpResponse::Ok().json(v)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/test")]
async fn do_stuff(req: HttpRequest) -> impl Responder {
    println!("{:?}", req);
    HttpResponse::Ok().body("Hello Test")
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
            .wrap_fn(|mut reqm, srv| {   
                println!("Hi from start. You requested: {}", reqm.path());
                let obj = reqm.headers_mut();
                let auth = HeaderName::from_lowercase(b"custom-header").unwrap();   
                obj.insert(auth, HeaderValue::from_static("boo"));
                println!("Was there an error here");
                srv.call(reqm)
            })
            .service(hello)
            .service(echo)
            .service(get_data)
            .service(routes::route_function_example)
            .route("/hey", web::get().to(manual_hello))
            .service(routes::do_stuff)
            .service(routes::login::login)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
