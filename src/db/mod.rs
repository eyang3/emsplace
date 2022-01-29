use std::string;
use std::fmt;
use r2d2_postgres::postgres::{Client, NoTls};
use r2d2_postgres::PostgresConnectionManager;

use rusoto_core::Region;
use rusoto_s3::{S3, S3Client, PutObjectRequest};

use std::env;

extern crate lazy_static;

lazy_static! {
    pub static ref POOL: r2d2::Pool<PostgresConnectionManager<NoTls>> = {
        let host = env::var("EMSPLACE_HOST").unwrap();
        let user = env::var("EMSPLACE_USER").unwrap();
        let conn_string = match env::var("EMSPLACE_PASS") {
            Ok(value) => format!("host={} user={} password={} dbname=emsplace", host, user, value),
            Err(e) =>  format!("host={} user={} dbname=emsplace", host, user),
        };

        let manager = PostgresConnectionManager::new(
            conn_string
                .parse()
                .unwrap(),
            NoTls,
        );
        r2d2::Pool::new(manager).unwrap()
    };
}

lazy_static! {
    pub static ref s3_client: S3Client = {
        let host = match env::var("EMSPLACE_S3") {
            Ok(value) => value,
            Err(_) => "http://localhost:4566".to_string()
        };
        return S3Client::new(Region::Custom {
            name: "localstack".to_owned(),
            endpoint: host.to_owned(),
        });
    };   
}

lazy_static! {
    pub static ref bucket: String = {
        let ret_value =  match env::var("EMSPLACE_S3") {
            Ok(value) => {
                Box::new(value)
            },
            Err(_) => {
                let value = String::from("http://localhost:4566");
                return Box::new(value).to_string();
            }
        };
        return(ret_value).to_string();
    };
}

pub static create_user: &str  = "INSERT INTO users (email, password, salt) VALUES ($1, $2, $3)";
pub static query_user: &str = "SELECT * FROM users where email = $1";


