mod types;
mod routes;
mod db;

use actix_web::dev::Service;
use actix_web::http::{HeaderValue, HeaderName};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::env;
use http::Uri;

use rusoto_core::Region;
use rusoto_s3::{S3, S3Client, PutObjectRequest};


#[macro_use]
extern crate lazy_static;

#[get("/s3")]
async fn tests3() -> impl Responder {
    let s3_client = S3Client::new(Region::Custom {
        name: "blah".to_owned(),
        endpoint: "http://localhost:4566".to_owned(),
    });
    let result = s3_client.put_object(PutObjectRequest {
        bucket: String::from("test"),
        key: "text2.txt".to_string(),
        body: Some("Hello".to_owned().into_bytes().into()),
        ..Default::default()
    }).await;
    HttpResponse::Ok().body("Hey there!")
}

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
                let obj = reqm.headers_mut();
                let basic_auth_header = obj.get("Authorization");
                match basic_auth_header {
                    Some(value) => {
                        let (auth, user_str) = authenticate(value);                        obj.insert(auth, HeaderValue::from_str(user_str.as_str()).unwrap());

                    },
                    None => println!("No header")
                }
                srv.call(reqm)
            })
            .service(hello)
            .service(echo)
            .service(get_data)
            .service(routes::route_function_example)
            .route("/hey", web::get().to(manual_hello))
            .service(routes::do_stuff)
            .service(routes::login::login)
            .service(tests3)
            .service(routes::posts::get_posts)

    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

fn authenticate(value: &HeaderValue)-> (HeaderName, String) {
    let basic_auth: &str = value.to_str().unwrap();
    let token = basic_auth.split(' ').collect::<Vec<&str>>()[1];
    let user = routes::login::validate_jwt(token);
    let user_str = user.as_str();
    let auth = HeaderName::from_lowercase(b"whoami").unwrap();
    return (auth, user_str.to_string());
}
