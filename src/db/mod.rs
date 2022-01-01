use std::string;

use r2d2_postgres::postgres::{Client, NoTls};
use r2d2_postgres::PostgresConnectionManager;

extern crate lazy_static;

lazy_static! {
    pub static ref POOL: r2d2::Pool<PostgresConnectionManager<NoTls>> = {
        let manager = PostgresConnectionManager::new(
            "host=localhost user=iamspazzy dbname=emsplace"
                .parse()
                .unwrap(),
            NoTls,
        );
        r2d2::Pool::new(manager).unwrap()
    };
}

pub static create_user: &str  = "INSERT INTO users (email, password, salt) VALUES ($1, $2, $3)";
pub static query_user: &str = "SELECT * FROM users where email = $1";