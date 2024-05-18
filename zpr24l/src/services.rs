use crate::{AppState, TokenClaims};
use actix_web::{
    get, post,
    web::{Data, Json, ReqData},
    HttpResponse, Responder,
};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web::web;
use argonautica::{Hasher, Verifier};
use chrono::NaiveDateTime;
use hmac::{Hmac, Mac};
use jwt::SignWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use sqlx::{self, FromRow};
use serde_json::json;

#[derive(Deserialize)]
struct CreateUserBody {
    username: String,
    password: String,
}

#[derive(Serialize, FromRow)]
struct UserNoPassword {
    id: i32,
    username: String,
}

#[derive(Serialize, FromRow)]
struct AuthUser {
    id: i32,
    username: String,
    password: String,
}

#[derive(Deserialize)]
struct CreateGraphBody {
    title: String,
    content: String,
}

#[derive(Serialize, FromRow, Deserialize)]
struct Graph {
    id: i32,
    title: String,
    content: String,
    published_by: i32,
    published_on: Option<NaiveDateTime>,
}

#[derive(Serialize, FromRow)]
struct GraphId {
    id: i32,
}


#[post("/user")]
async fn create_user(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder {
    let user: CreateUserBody = body.into_inner();

    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(user.password)
        .with_secret_key(hash_secret)
        .hash()
        .unwrap();

    match sqlx::query_as::<_, UserNoPassword>(
        "SELECT id, username FROM users WHERE username = $1",
    )
    .bind(user.username.clone())
    .fetch_optional(&state.db)
    .await
    {
        Ok(Some(_)) => {
            return HttpResponse::Conflict().json("User with that username already exists")
        }
        Err(error) => return HttpResponse::InternalServerError().json(format!("{:?}", error)),
        _ => (),
    }
    match sqlx::query_as::<_, UserNoPassword>(
        "INSERT INTO users (username, password)
        VALUES ($1, $2)
        RETURNING id, username",
    )
    .bind(user.username)
    .bind(hash)
    .fetch_one(&state.db)
    .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
    }
}

#[post("/auth")]
async fn basic_auth(state: Data<AppState>, credentials: BasicAuth) -> impl Responder {
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set!")
            .as_bytes(),
    )
    .unwrap();
    let username = credentials.user_id();
    let password = credentials.password();

    match password {
        None => HttpResponse::Unauthorized().json("Must provide username and password"),
        Some(pass) => {
            match sqlx::query_as::<_, AuthUser>(
                "SELECT id, username, password FROM users WHERE username = $1",
            )
            .bind(username.to_string())
            .fetch_one(&state.db)
            .await
            {
                Ok(user) => {
                    let hash_secret =
                        std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
                    let mut verifier = Verifier::default();
                    let is_valid = verifier
                        .with_hash(user.password)
                        .with_password(pass)
                        .with_secret_key(hash_secret)
                        .verify()
                        .unwrap();

                    if is_valid {
                        let claims = TokenClaims { id: user.id };
                        let token_str = claims.sign_with_key(&jwt_secret).unwrap();
                        HttpResponse::Ok().json(json!({ "auth_token": token_str }))
                    } else {
                        HttpResponse::Unauthorized().json("Incorrect username or password")
                    }
                }
                Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
            }
        }
    }
}

#[post("/graph")]
async fn create_graph(
    state: Data<AppState>,
    req_user: Option<ReqData<TokenClaims>>,
    body: Json<CreateGraphBody>,
) -> impl Responder {
    match req_user {
        Some(user) => {
            let graph: CreateGraphBody = body.into_inner();

            match sqlx::query_as::<_, Graph>(
                "INSERT INTO graphs (title, content, published_by)
                VALUES ($1, $2, $3)
                RETURNING id, title, content, published_by, published_on",
            )
            .bind(graph.title)
            .bind(graph.content)
            .bind(user.id)
            .fetch_one(&state.db)
            .await
            {
                Ok(graphs) => HttpResponse::Ok().json(graphs),
                Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
            }
        }
        _ => HttpResponse::Unauthorized().json("Unable to verify identity"),
    }
}



#[get("/graph/{id}")]
async fn get_graph_by_id(
    state : Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    match sqlx::query_as::<_, Graph>(
        "SELECT id, title, content, published_by, published_on
        FROM graphs
        WHERE id = $1",
    )
    .bind(id.into_inner())
    .fetch_one(&state.db)
    .await
    {
        Ok(graph) => HttpResponse::Ok().json(graph),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
    }
}


#[get("/user/{id}/graphs")]
async fn get_user_graphs(
    state: Data<AppState>,
    id: web::Path<i32>,
) -> impl Responder {
    match sqlx::query_as::<_, GraphId>(
        "SELECT id FROM graphs WHERE published_by = $1",
    )
    .bind(id.into_inner())
    .fetch_all(&state.db)
    .await
    {
        Ok(graphs) => HttpResponse::Ok().json(graphs),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
    }
}