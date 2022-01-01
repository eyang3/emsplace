use super::types; 
use super::db;

use std::io::Write;
use std::str;
use std::fs::File;

use actix::fut::future::result;
use data_url::{DataUrl};
use nanoid::nanoid;

use actix_multipart::Multipart;
use actix_web::web::{Bytes, Json};
use actix_web::{HttpResponse, post, HttpRequest, Responder};
use futures::{StreamExt, TryStreamExt};

use data_encoding::HEXUPPER;
use r2d2_postgres::postgres::row::RowIndex;
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

pub mod login;

pub async fn save_file(mut payload: Multipart, file_path: String) -> Option<bool> {    
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let name = content_type.get_name().unwrap();
        if name == "image" {
            let file_content = Some(field.map(|chunk| chunk.unwrap()).collect::<Vec<Bytes>>().await);
            match file_content {
                Some(vec) => { 
                    base64_url_to_file(vec);
                },
                None => println!("There is an Error")
            }
        }
    }
    Some(true)
}

fn base64_url_to_file(vec: Vec<Bytes>) {
    let content = vec.concat();
    let val = str::from_utf8(&content);
    let base64 = val.unwrap();
    let url = DataUrl::process(base64).unwrap();
    let (body, _fragment) = url.decode_to_vec().unwrap();
    let file_type =  &url.mime_type().subtype;
    let root = nanoid!(10);
    let file_name = root + "." + file_type;
    let mut file = File::create(file_name).unwrap();
    file.write_all(&body);
}

#[post("/upload_image")]
pub async fn route_function_example(
    mut payload: Multipart
) -> Result<HttpResponse, HttpResponse> {
    let upload_status = save_file(payload, "filename.jpg".to_string()).await;
    match upload_status {
        Some(true) => {
            Ok(HttpResponse::Ok()
                .content_type("text/plain")
                .body("update_succeeded"))
        }
        _ => Ok(HttpResponse::BadRequest()
            .content_type("text/plain")
            .body("update_failed")),
    }
}

#[post("/signup")]
async fn do_stuff(req: HttpRequest, info: Json<types::UserSignup>) ->  HttpResponse  {
    let mut client = db::POOL.get().unwrap();

    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let rng = rand::SystemRandom::new();
    let mut salt = [0u8; 16];
    rng.fill(&mut salt);

    let password = &info.password;
    let username = &info.email;

    let n_iter = NonZeroU32::new(8).unwrap();

    let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );

    let salt_str = HEXUPPER.encode(&salt);
    let pw_hash = HEXUPPER.encode(&pbkdf2_hash);
    let result = client.query(db::create_user, &[&username, &pw_hash, &salt_str]);
    match result {
        Ok(_row) => {
            let ret = types::APIResponse{result: "success", message: "succces".to_string()};
            return HttpResponse::Ok().json(ret);
        },
        Err(error) => {
            let ret = types::APIResponse{result: "error", message: error.to_string()};
            return HttpResponse::BadRequest().json(ret);
        }
    };
}
