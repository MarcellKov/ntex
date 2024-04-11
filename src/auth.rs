pub mod models;
pub mod utilities;

use chrono::{Datelike, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use cookie::{
    time::{format_description::modifier::OffsetHour, Date, Duration, OffsetDateTime, UtcOffset},
    Cookie,
};
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use models::Claims;
use ntex::http::header::{self};
use ntex::{
    http::HttpMessage,
    web::{self, types::Json, HttpRequest, HttpResponse},
};
use std::{env, io::Read, ops::Add, path};
use utilities::{extractcookie, fallback};
extern crate dotenv;

#[web::post("/create-jwt/{uname}/{role}")]
async fn generate_token(path: web::types::Path<(String, String)>) -> impl web::Responder {
    let claims = models::Claims {
        name_claim: path.0.clone(),
        usertype_claim: path.1.clone(),
        exp: NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 8, 8).unwrap(),
            NaiveTime::from_hms_opt(8, 8, 8).unwrap(),
        )
        .timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(std::env::var("secret").unwrap().as_bytes()),
    );

    let suti = Cookie::build(("mycookie", token.clone().unwrap()))
        .http_only(true)
        .path("/")
        .expires(OffsetDateTime::now_utc() + Duration::seconds(5000));
    HttpResponse::Ok().cookie(suti).body(token.unwrap())
}

async fn getinfo(req: HttpRequest) -> impl web::Responder {
    let resp = extractcookie(req, "mycookie".to_owned()).await;
    HttpResponse::Ok().body(resp.unwrap().claims.name_claim)
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    web::HttpServer::new(|| {
        web::App::new()
            .service(generate_token)
            .route("/extract-jwt", web::get().to(getinfo))
            .route("/fallback", web::get().to(fallback))
    })
    .bind(("0.0.0.0", 8090))?
    .run()
    .await
}
