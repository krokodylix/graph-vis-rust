use crate::{AppState, TokenClaims};

use actix_web::web;
use actix_web::{
    get, post,
    web::{Data, Json, ReqData},
    HttpResponse, Responder,
};

use argonautica::{Hasher, Verifier};

use chrono::NaiveDateTime;

use hmac::{Hmac, Mac};
use jwt::SignWithKey;

use serde::{Deserialize, Serialize};
use serde_json::json;

use sha2::Sha256;

use sqlx::{self, FromRow};

use rand::Rng;


// CODE IN THIS FILE IS RESPONSIBLE FOR HANDLING THE API ROUTES AND THEIR CORRESPONDING FUNCTIONS


// Define a struct for the body of the create user request.
#[derive(Deserialize)]
struct CreateUserBody {
    username: String,
    password: String,
}

// Define a struct for a user without a password.
#[derive(Serialize, FromRow)]
struct UserNoPassword {
    id: i32,
    username: String,
}

// Define a struct for an authenticated user.
#[derive(Serialize, FromRow)]
struct AuthUser {
    id: i32,
    username: String,
    password: String,
}

// Define a struct for the body of the create graph request.
#[derive(Deserialize)]
struct CreateGraphBody {
    title: String,
    content: String,
}

// Define a struct for a graph.
#[derive(Serialize, FromRow, Deserialize)]
struct Graph {
    id: i32,
    title: String,
    content: String,
    published_by: i32,
    published_on: Option<NaiveDateTime>,
}

// Define a struct for a simplified graph representation.
#[derive(Serialize, FromRow)]
struct GraphSimple {
    id: i32,
    title: String,
}

// Define a struct for the body of the random graph request.
#[derive(Deserialize)]
struct RandomGraphBody {
    vertices: i32,
    edges: i32,
}

// Endpoint to register a new user.
#[post("/api/register")]
async fn create_user(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder {

    // check if the username or password is empty.
    if body.username.is_empty() || body.password.is_empty() {
        return HttpResponse::BadRequest().json(json!({ "error": "Username or password is empty" }));
    }

    let user: CreateUserBody = body.into_inner();

    // Hash the user's password.
    let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(user.password)
        .with_secret_key(hash_secret)
        .hash()
        .unwrap();

    // Check if a user with the same username already exists.
    match sqlx::query_as::<_, UserNoPassword>("SELECT id, username FROM users WHERE username = $1")
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

    // Insert the new user into the database.
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

// Endpoint to authenticate a user and return a JWT token.
#[post("/api/auth")]
async fn basic_auth(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder {

    // Check if the username or password is empty.
    if body.username.is_empty() || body.password.is_empty() {
        return HttpResponse::BadRequest().json(json!({ "error": "Username or password is empty" }));
    }
    
    // Retrieve the JWT secret key.
    let jwt_secret: Hmac<Sha256> = Hmac::new_from_slice(
        std::env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set!")
            .as_bytes(),
    )
    .unwrap();
    let user: CreateUserBody = body.into_inner();

    // Fetch the user's details from the database.
    match sqlx::query_as::<_, AuthUser>(
        "SELECT id, username, password FROM users WHERE username = $1",
    )
    .bind(user.username.clone())
    .fetch_one(&state.db)
    .await
    {
        Ok(auth_user) => {
            // Verify the user's password.
            let hash_secret = std::env::var("HASH_SECRET").expect("HASH_SECRET must be set!");
            let mut verifier = Verifier::default();
            let is_valid = verifier
                .with_hash(auth_user.password)
                .with_password(user.password)
                .with_secret_key(hash_secret)
                .verify()
                .unwrap();

            if is_valid {
                // Create JWT token if credentials are valid.
                let claims = TokenClaims { id: auth_user.id };
                let token_str = claims.sign_with_key(&jwt_secret).unwrap();
                HttpResponse::Ok().json(json!({ "auth_token": token_str }))
            } else {
                HttpResponse::Unauthorized().json(json!({ "error": "Invalid credentials" }))
            }
        }
        Err(error) => {
            HttpResponse::InternalServerError().json(json!({ "error": format!("{:?}", error) }))
        }
    }
}

// Endpoint to create a new graph.
#[post("/api/graph")]
async fn create_graph(
    state: Data<AppState>,
    req_user: Option<ReqData<TokenClaims>>,
    body: Json<CreateGraphBody>,
) -> impl Responder {
    match req_user {
        Some(user) => {
            let graph: CreateGraphBody = body.into_inner();

            // Insert the new graph into the database.
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

// Endpoint to get a graph by its ID.
#[get("/api/graph/{id}")]
async fn get_graph_by_id(state: Data<AppState>, id: web::Path<i32>) -> impl Responder {
    // Fetch the graph from the database.
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

// Endpoint to get all graphs created by a user.
#[get("/api/user/{id}/graphs")]
async fn get_user_graphs(state: Data<AppState>, id: web::Path<i32>) -> impl Responder {
    // Fetch the user's graphs from the database.
    match sqlx::query_as::<_, GraphSimple>(
        "SELECT id, title
        FROM graphs
        WHERE published_by = $1",
    )
    .bind(id.into_inner())
    .fetch_all(&state.db)
    .await
    {
        Ok(graphs) => HttpResponse::Ok().json(graphs),
        Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
    }
}

// Endpoint to generate a random graph.
#[post("/api/randomgraph")]
async fn random_graph(body: Json<RandomGraphBody>) -> impl Responder {
    let random_graph: RandomGraphBody = body.into_inner();
    let mut rng = rand::thread_rng();
    let mut graph = String::new();
    
    // Generate random edges for the graph.
    for _ in 0..random_graph.edges {
        let v1 = rng.gen_range(1..random_graph.vertices + 1);
        let v2 = loop {
            let v2 = rng.gen_range(1..random_graph.vertices + 1);
            if v2 != v1 {
                break v2;
            }
        };
        graph.push_str(&format!("{}-{},", v1, v2));
    }
    graph.pop();

    // Return the generated graph as a response.
    HttpResponse::Ok().json(json!({ "graph": graph }))
}
