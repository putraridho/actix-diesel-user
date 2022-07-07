mod api;
mod connection;
mod model;
mod schema;

#[macro_use]
extern crate diesel;
extern crate actix_web;
extern crate dotenv;

use crate::connection::establish_pool_connection;
use actix_web::{middleware, web, App, HttpServer};
use api::user::{add_user, get_user_by_id, get_users, remove_user, update_user};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let pool = establish_pool_connection();

    log::info!("Starting HTTP server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(get_users)
            .service(get_user_by_id)
            .service(add_user)
            .service(update_user)
            .service(remove_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
