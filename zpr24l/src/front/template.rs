use actix_web::{get, HttpResponse, Responder};

use lazy_static::lazy_static;
use tera::{Context, Tera};

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
    let page = TEMPLATES.render("auth/login.html", &context).unwrap();
    HttpResponse::Ok().body(page)
}

#[get("/addgraph")]
async fn addgraph() -> impl Responder {
    let context = Context::new();
    let page = TEMPLATES.render("graphs/addgraph.html", &context).unwrap();
    HttpResponse::Ok().body(page)
}

#[get("/register")]
async fn register() -> impl Responder {
    let context = Context::new();
    let page = TEMPLATES.render("auth/register.html", &context).unwrap();
    HttpResponse::Ok().body(page)
}

#[get("/logout")]
async fn logout() -> impl Responder {
    let context = Context::new();
    let page = TEMPLATES.render("auth/logout.html", &context).unwrap();
    HttpResponse::Ok().body(page)
}

#[get("/usergraphs")]
async fn usergraphs() -> impl Responder {
    let context = Context::new();
    let page = TEMPLATES
        .render("graphs/usergraphs.html", &context)
        .unwrap();
    HttpResponse::Ok().body(page)
}
