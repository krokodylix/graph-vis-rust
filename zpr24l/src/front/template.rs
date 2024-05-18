// module that uses ascix web and returns templates

use actix_web::{web, App, HttpServer, Responder, HttpResponse, get, post, middleware::Logger, middleware::NormalizePath};
use actix_web::web::Data;
use actix_web_httpauth::{extractors::{bearer::{self, BearerAuth}, AuthenticationError}, middleware::HttpAuthentication};

use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use lazy_static::lazy_static;
use tera::{Tera, Context};


lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "templates/**/*";
        let tera = Tera::new(source).unwrap();
        tera
    };
}
#[get("/")]
async fn root_dir() -> impl Responder {
    let context = Context::new();
    let page = TEMPLATES.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(page)
}

#[get("/login")]
async fn login() -> impl Responder {
    let context = Context::new();
    let page = TEMPLATES.render("login.html", &context).unwrap();
    HttpResponse::Ok().body(page)
}

#[get("/addgraph")]
async fn addgraph() -> impl Responder {
    let context = Context::new();
    let page = TEMPLATES.render("addgraph.html", &context).unwrap();
    HttpResponse::Ok().body(page)
}

#[get("/register")]
async fn register() -> impl Responder {
    let context = Context::new();
    let page = TEMPLATES.render("register.html", &context).unwrap();
    HttpResponse::Ok().body(page)
}

#[get("/logout")]
async fn logout() -> impl Responder {
    let context = Context::new();
    let page = TEMPLATES.render("logout.html", &context).unwrap();
    HttpResponse::Ok().body(page)
}