#[macro_use]
extern crate rocket;
use rocket::http::uri::Origin;
use rocket::serde::json::{json, Value};

const API_ENDPOINT: Origin<'static> = uri!("/api");

#[get("/")]
fn index() -> String {
    String::from("Hello, world!")
}



#[get("/hellozpr")]
fn json() -> Value {
    json!({
        "status": "ok",
        "message": "Hello, world!"
    })
}

#[launch]
fn rocket() -> _{
    rocket::build()
        .mount("/", routes![index])
        .mount(API_ENDPOINT, routes![json])
}