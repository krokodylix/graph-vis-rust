use actix_web::{
    dev::ServiceRequest,
    error::Error,
    web::{self, Data},
    App, HttpMessage, HttpServer,
};

use dotenv::dotenv;

use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use actix_web_httpauth::{
    extractors::{
        bearer::{self, BearerAuth},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};

use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use actix_cors::Cors;

mod api {
    pub mod services;
}
use api::services::{basic_auth, create_graph, create_user, get_graph_by_id, get_user_graphs, random_graph};

mod front {
    pub mod template;
}
use front::template::{addgraph, login, logout, register, root_dir, usergraphs};


// CODE IN THIS FILE IS RESPONSIBLE FOR SETTING UP AND RUNNING THE ACTIX WEB SERVER


// Struct to hold the application state, including the database connection pool.
pub struct AppState {
    db: Pool<Postgres>,
}

// Struct to represent the claims extracted from a JWT token.
#[derive(Serialize, Deserialize, Clone)]
pub struct TokenClaims {
    id: i32,
}

// Function to validate the JWT token provided in the request.
// If valid, the token claims are added to the request extensions for further use.
async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let jwt_secret: String = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set!");
    let key: Hmac<Sha256> = Hmac::new_from_slice(jwt_secret.as_bytes()).unwrap();
    let token_string = credentials.token();

    let claims: Result<TokenClaims, &str> = token_string
        .verify_with_key(&key)
        .map_err(|_| "Invalid token");

    match claims {
        Ok(value) => {
            req.extensions_mut().insert(value);
            Ok(req)
        }
        Err(_) => {
            let config = req
                .app_data::<bearer::Config>()
                .cloned()
                .unwrap_or_default()
                .scope("");

            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

// Main function to set up and run the Actix web server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file.
    dotenv().ok();

    // Retrieve the database URL from the environment variables and create a connection pool.
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error building a connection pool");

    // Configure and start the HTTP server.
    HttpServer::new(move || {
        // Configure CORS to allow any origin, method, and header.
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        // Configure the bearer authentication middleware.
        let bearer_middleware = HttpAuthentication::bearer(validator);

        // Set up the Actix web application with the necessary services and middleware.
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .wrap(cors)
            .service(basic_auth)
            .service(create_user)
            .service(get_graph_by_id)
            .service(get_user_graphs)
            .service(root_dir)
            .service(login)
            .service(addgraph)
            .service(register)
            .service(logout)
            .service(usergraphs)
            .service(random_graph)
            .service(web::scope("").wrap(bearer_middleware).service(create_graph))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
