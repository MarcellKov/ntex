use cookie::Cookie;
use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
use ntex::{
    http::HttpMessage,
    web::{self, HttpRequest, HttpResponse},
};
use sea_orm::{
    self, sea_query::extension::sqlite, ConnectOptions, Database, DatabaseConnection, DbErr,
};
use serde::{Deserialize, Serialize};

use crate::models::{self, Claims};
#[derive(Deserialize, Serialize, Debug)]
pub struct MSG {
    pub msg: String,
}
pub async fn auth(req: HttpRequest) -> HttpResponse {
    match req.cookie("mycookie") {
        None => HttpResponse::Unauthorized()
            .header("Location", "/fallback")
            .finish(),
        Some(i) => match jsonwebtoken::decode::<models::Claims>(
            i.value(),
            &DecodingKey::from_secret(std::env::var("secret").unwrap().as_bytes()),
            &Validation::default(),
        ) {
            Err(_) => HttpResponse::Unauthorized()
                .header("Location", "/fallback")
                .finish(),
            Ok(i) => match i.claims.usertype_claim {
                String => HttpResponse::Ok().finish(),
            },
        },
    }
}

pub async fn fallback() -> impl web::Responder {
    HttpResponse::Ok().body("<h2>Please log in order to use the page<h2>")
}

pub async fn extractcookie(
    req: HttpRequest,
    name: String,
) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let cookie: Option<Cookie<'_>> = req.cookie(&name);
    let resp = decode::<models::Claims>(
        &String::from(cookie.as_ref().unwrap().value()),
        &DecodingKey::from_secret(std::env::var("secret").unwrap().as_bytes()),
        &Validation::default(),
    );
    resp
}

pub async fn estabilish_connection() -> Result<DatabaseConnection, DbErr> {
    let opt: ConnectOptions = ConnectOptions::new(std::env::var("DATABASE_URL").unwrap().as_str());
    let db = Database::connect(opt).await;
    match db {
        Ok(i) => return Ok(i),
        Err(i) => return Err(i),
    }
}
