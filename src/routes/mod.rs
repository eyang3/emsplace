use crate::db::save_post_to_db;

use super::types; 
use super::db;

use std::io::Write;
use std::str;
use std::fs::File;

use data_url::{DataUrl};
use nanoid::nanoid;

use actix_multipart::Multipart;
use actix_web::web::{Bytes, Json};
use actix_web::{HttpResponse, post, HttpRequest};
use futures::{StreamExt, TryStreamExt};

use data_encoding::HEXUPPER;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

use rusoto_s3::{S3, PutObjectRequest};

pub mod login;

pub async fn save_file(mut payload: Multipart, whoami: &str) -> &'static  str {   
    let mut entry = types::ImageUpload::default();
    if(whoami != "No User") {
        entry.userid = whoami.parse::<i32>().unwrap();
    } else {
        entry.userid = -1;
    }
    let mut res = "";
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let name = content_type.get_name().unwrap();
        let file_content = Some(field.map(|chunk| chunk.unwrap()).collect::<Vec<Bytes>>().await);
        println!("Something happened here");
        if name == "caption" {
            let content = file_content.unwrap().concat();
            let message = std::str::from_utf8(&content).unwrap();
            entry.caption = message.to_string();

        }
        else if name == "image" {
            res = match file_content {
                Some(vec) => {
                    // base64_url_to_file(vec).await;
                    let value = base64_url_to_s3(vec).await;
                    entry.filename = value.to_string();
                    println!("{:?}", value);
                    "Okay"
                },
                None => "Error"
            };
        }

    }
    save_post_to_db(entry).await;
    return res;
}

async fn base64_url_to_s3(vec: Vec<Bytes>)-> Box<std::string::String> {
    let content = vec.concat();
    let val = str::from_utf8(&content);
    let base64 = val.unwrap();
    let url = DataUrl::process(base64).unwrap();
    let (body, _fragment) = url.decode_to_vec().unwrap();
    let m = md5::compute(body.to_owned());
    let ovec = format!("{:x}", m);
 
    let file_type =  &url.mime_type().subtype;
    if file_type == "jpg" || file_type == "png" {
        let file_name = ovec + "." + file_type;
        let result = db::s3_client.put_object(PutObjectRequest {
            bucket: String::from("test"),
            key: file_name.to_owned(),
            body: Some(body.to_owned().into()),
            ..Default::default()
        }).await;
        return Box::new(file_name);
    }
    return Box::new("Error".to_string());
}

async fn base64_url_to_file(vec: Vec<Bytes>)-> &'static str {
    let content = vec.concat();
    let val = str::from_utf8(&content);
    let base64 = val.unwrap();
    let url = DataUrl::process(base64).unwrap();
    let (body, _fragment) = url.decode_to_vec().unwrap();
    let file_type =  &url.mime_type().subtype;
    if file_type == "jpg" || file_type == "png" {
        let root = nanoid!(10);
        let file_name = root + "." + file_type;
        let mut file = File::create(file_name).unwrap();
        file.write_all(&body);
        return("valid image file");
    }
    return("invalid image filetype");
}

#[post("/upload_image")]
pub async fn route_function_example(req: HttpRequest,
    mut payload: Multipart
) -> Result<HttpResponse, HttpResponse> {
    let whoami = req.headers().get("whoami").unwrap().to_str();
    let upload_status = save_file(payload, &whoami.unwrap()).await;
    println!("{:?}", upload_status);
    Ok(HttpResponse::Ok()
                .content_type("text/plain")
                .body(upload_status))
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
