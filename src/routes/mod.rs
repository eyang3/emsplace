use std::io::Write;
use std::str;
use std::fs::File;

use data_url::{DataUrl};
use nanoid::nanoid;

use actix_multipart::Multipart;
use actix_web::web::{Bytes};
use actix_web::{HttpResponse, post};
use futures::{StreamExt, TryStreamExt};

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