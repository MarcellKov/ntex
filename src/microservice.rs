pub mod models;
use cookie::time::Duration;
use dotenv::dotenv;
use entity::post;
use jsonwebtoken::{DecodingKey, Validation};
use ntex::http;
use ntex::util::HashMap;
use ntex::web::types::Json;
use ntex::web::Responder;
use ntex::{
    http::{body::ResponseBody, HttpMessage},
    web::{self, error::JsonError, App, HttpRequest, HttpResponse, WebRequest, WebResponse},
    Service,
};
use ntex_cors::Cors;
use ntex_http::StatusCode;
use ntex_session::{CookieSession, Session, UserSession};
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::{self, Set};
use serde::Serialize;
use std::any::TypeId;
use std::{env, io::Read, ops::Add};
use utilities::{auth, estabilish_connection, MSG};
extern crate dotenv;

pub mod utilities;
async fn rest(req: HttpRequest) -> impl web::Responder {
    let authres = utilities::auth(req).await;

    if authres.status() == StatusCode::OK {
        HttpResponse::Ok().body("item")
    } else {
        HttpResponse::Found()
            .header("Location", "/fallback")
            .finish()
    }
}
async fn create() -> impl web::Responder {
    let db = estabilish_connection().await;
    match db {
        Err(i) => HttpResponse::InternalServerError().body("Failed to estabilish connection to db"),
        Ok(_) => {
            let post = entity::post::ActiveModel {
                id: Set(1),
                text: Set("asd".to_owned()),
                title: Set("vegre".to_owned()),
            };
            let vegso = post.insert(&db.unwrap()).await.unwrap();
            HttpResponse::Ok()
            .content_type("application/json")
            .json(&vegso)
        }
    }
}

async fn sessioncookie(ses: Session,req:HttpRequest) -> impl web::Responder {
    ses.set("csrf", "nagyondurvacsrf token").unwrap();
    req.get_session().set("csrf", "nagyondurvacsrf token").unwrap();
    

    HttpResponse::Ok().content_type("application/json").json(&MSG{msg:"cookie session succesfully set, furaag".to_string()})
}

async fn getsessioncookie(ses: Session) -> impl web::Responder {
    let x = ses.get::<String>("csrf");
    match x {
        Err(i) => HttpResponse::InternalServerError().body(i.to_string()),
        Ok(i) => match i {
            None => HttpResponse::InternalServerError().finish(),
            Some(i) => HttpResponse::Ok().body(i),
        },
    }
}

async fn anticsrf(ses: Session, req: HttpRequest) -> impl web::Responder {
    let session = req.get_session().get::<String>("csrf");
    match session {
        Err(i) => HttpResponse::InternalServerError()
        .content_type("application/json")
        .json(&MSG { msg: i.to_string() }),
        Ok(i) => match i {
            None => HttpResponse::BadGateway()
            .content_type("application/json")
            .json(&MSG {
                msg: "ures csrf token".to_string(),
            }),
            Some(i) => {
                if i == ses.get::<String>("csrf").unwrap().unwrap() {
                    HttpResponse::Ok()
                    .content_type("application/json")
                    .json(&MSG {
                        msg: "valid csrf token".to_string(),
                    })
                } else {
                    HttpResponse::BadRequest()
                    .content_type("application/json")
                    .json(&MSG {
                        msg: "invalid csrf token".to_string(),
                    })
                }
            }
        },
    }
}
async fn probacska() -> impl web::Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .json(&MSG {
            msg: "A".to_string(),
        })
}

#[ntex::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    web::HttpServer::new(|| {
        web::App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:3000")
                    .supports_credentials()
                    .finish(),
            )
            .wrap(
                CookieSession::signed(&[0; 32])
                    .expires_in_time(Duration::minutes(5))
                    .name("sajat-session")
                    .same_site(cookie::SameSite::None)
            )
            .route("/auth", web::get().to(auth))
            .route("/rest", web::get().to(rest))
            .route("/create", web::get().to(create))
            .route("/fallback", web::get().to(utilities::fallback))
            .route("/session", web::get().to(sessioncookie))
            .route("/getcsrf", web::get().to(getsessioncookie))
            .route("/anticsrf", web::get().to(anticsrf))
            .route("/probacska", web::get().to(probacska))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
