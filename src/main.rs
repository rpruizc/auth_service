#[macro_use]
extern crate diesel;
extern crate serde_json;
extern crate lettre;
extern crate native_tls;
extern crate cookie;

use actix_cors::Cors;
use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{middleware, web, App, HttpServer, http::header};
use diesel::{
    prelude::*,
    r2d2::{self, ConnectionManager}
};

mod email_service;
mod errors;
mod models;
mod password_handler;
mod register_handler;
mod schema;
mod templates;
mod utils;
mod vars;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    // create a database connection pool
    let manager = ConnectionManager::<PgConnection>::new(vars::database_url());
    let pool: models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create a database connection pool.");

    // Start the http server
    HttpServer::new( move || {
        App::new()
            .data(pool.clone())
            // enable logger
            .wrap(middleware::Logger::default())
            // enable sessions
            .wrap(
                CookieSession::signed(&[0, 32])
                .domain(vars::domain_url().as_str())
                .name("auth")
                .secure(false))
            .wrap(Cors::new()
                .allowed_methods(vec![ "DELETE", "GET", "OPTIONS", "POST",])
                .max_age(3600)
                .finish())
            .service(Files::new("/assets", "./templates/assets"))

            // Routes
            .service(
                web::scope("/")
                    .service(
                        web::resource("/register")
                            .route(web::get().to(register_handler::show_confirmation_form))
                            .route(web::post().to(register_handler::send_confirmation)),
                    )
                    .service(
                        web::resource("/register/{path_id}")
                            .route(web::get().to(password_handler::show_password_form))
                            .route(web::post().to(password_handler::create_account)),
                    )
                    .route("/register2/{path_id}", web::post().to(password_handler::create_account_for_browser))
                    .route("/register2", web::post().to(register_handler::send_confirmation_for_browser)),
            )
        })
        .bind(format!("{}:{}", vars::domain(), vars::port()))?
        .run()
        .await
}