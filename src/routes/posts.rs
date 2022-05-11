use super::types; 
use super::db;

use std::collections::HashMap;
use std::num::NonZeroU32;
use std::env;

use actix_web::web::Query;
use actix_web::web::{Json};
use actix_web::{HttpResponse, post, get, HttpRequest};
use ring::{digest, pbkdf2};
use db::{get_new_posts, get_top_posts, POOL};
use chrono::{Duration, Utc};

#[get("/posts")]
pub async fn get_posts(req: HttpRequest) -> Result<HttpResponse, HttpResponse> {
    let mut client = POOL.get().unwrap();
    let whoami = req.headers().get("whoami").unwrap().to_str();
    println!("{:?}", whoami);
    let page_info = Query::<HashMap<String, u32>>::from_query(req.query_string()).unwrap();

    let offset = match page_info.get("offset") {
        Some(value) => *value,
        None => 0,
    };
    let limit = match page_info.get("limit") {
        Some(value) => *value,
        None => 10,
    };
    let top = match page_info.get("top") {
        Some(value) => true,
        None => false,
    };
    let dt = Utc::now() - Duration::days(60);
    let dt_stamp = dt.timestamp();
    let query = match top {
        true => client.query(get_top_posts, &[&dt_stamp, &limit, &offset]),
        false => client.query(get_new_posts, &[&limit, &offset]),
    } ;
    println!("{:?}", query);

    Ok(HttpResponse::Ok()
                .content_type("text/plain")
                .body("upload_status"))
}
