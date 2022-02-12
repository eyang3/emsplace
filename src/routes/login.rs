
use super::types; 
use super::db;

use std::num::NonZeroU32;
use std::env;

use actix_web::web::{Json};
use actix_web::{HttpResponse, post, HttpRequest};
use data_encoding::HEXUPPER;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use ring::{digest, pbkdf2};

pub fn validate_jwt(token: &str) -> String {
    let secret = match env::var("EMSPLACE_SECRET") {
        Ok(value) => value,
        Err(E) => "DEFAULT".to_string()
    };
    let token_message = decode::<types::Claims>(&token, 
        &DecodingKey::from_secret(secret.as_bytes()), 
        &Validation::new(Algorithm::HS256));
    let msg = match token_message {
        Ok(value) => value.claims.userid.to_string(),
        Err(e) => {
            "No User".to_string()
        }
    };
    return msg;
}
fn generate_jwt(userid: i32, username: &str) -> String{
    let secret = match env::var("EMSPLACE_SECRET") {
        Ok(value) => value,
        Err(E) => "DEFAULT".to_string()
    };
    // no expiration date so far TODO: Change to a monthly expiration
    let claims = types::Claims { userid: userid, username: username.to_string(), exp: 10000000000};
    let value = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).unwrap();
    return value;
}

#[post("/login")]
async fn login(req: HttpRequest, info: Json<types::UserSignup>) ->  HttpResponse  {
    let mut client = db::POOL.get().unwrap();
    let password = &info.password;
    let username = &info.email;
    let result = client.query(db::query_user, &[&username]);
    match result {
        Ok(rows) => {
            const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
            let mut pbkdf2_hash = [0u8; CREDENTIAL_LEN];
            if(rows.len() == 0) {
                let ret = types::APIResponse{result: "error", message: "User Not Found"};
                return HttpResponse::BadRequest().json(ret);
            } 
            let mut salt_string: &str = "";
            let mut password_hash: &str = "";
            let mut userid: i32 = 0;

            for r in &rows {
                userid = r.get("id");
                salt_string = r.get("salt");
                password_hash = r.get("password");
            }
            let salt = HEXUPPER.decode(salt_string.as_bytes()).unwrap();
            pbkdf2::derive(
                pbkdf2::PBKDF2_HMAC_SHA512,
                NonZeroU32::new(8).unwrap(),
                &salt,
                password.as_bytes(),
                &mut pbkdf2_hash,
            );
            let pw_hash = HEXUPPER.encode(&pbkdf2_hash);
            if(pw_hash != password_hash) {
                let ret = types::APIResponse{result: "Unauthorized", message: "Cannot Log-in"};
                return HttpResponse::Forbidden().json(ret);
            } else {
                let ret = types::APIResponse{result: "Success", message: generate_jwt(userid, username)};
                return HttpResponse::Ok().json(ret);
            }
        },
        Err(e) => {
            let ret = types::APIResponse{result: "error", message: e.to_string()};
            return HttpResponse::BadRequest().json(ret);
        }
    }
}
