use std::io::Write;
use url::Url;
use std::str;
use base64::{decode};
use std::fs::File;


use data_url::{DataUrl, mime};
use nanoid::nanoid;


use actix_multipart::Multipart;
use actix_web::web::{Bytes, post};
use actix_web::{middleware, web, get, post, App, Error, HttpResponse, HttpServer};
use futures::{StreamExt, TryStreamExt};


pub async fn save_file(mut payload: Multipart, file_path: String) -> Option<bool> {
    // iterate over multipart stream
    
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
        println!("{}", content_type);
        //let filename = content_type.get_filename().unwrap();
        let filepath = format!(".{}", file_path);
        /*
        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&data).map(|_| f))
                .await
                .unwrap();
        }
        */
    }
    Some(true)
}

fn base64_url_to_file(vec: Vec<Bytes>) {
    let content = vec.concat();
    let val = str::from_utf8(&content);
    let base64 = val.unwrap();
    let url = DataUrl::process(base64).unwrap();
    let (body, fragment) = url.decode_to_vec().unwrap();
    let fileType =  &url.mime_type().subtype;
    let root = nanoid!(10);
    let fileName = root + "." + fileType;
    let mut file = File::create(fileName).unwrap();
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