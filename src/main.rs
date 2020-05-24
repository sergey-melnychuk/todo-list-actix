use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::io;

mod model;
mod repo;
mod handler;
mod config;

use crate::config::Config;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let config = Config::env().unwrap();
    let pool = config.db.create_pool(tokio_postgres::NoTls).unwrap();
    let address = format!("{}:{}", config.server.host, config.server.port);

    HttpServer::new(move || {
            App::new()
                .data(pool.clone())
                .route("/", web::get().to(handler::status))
                .route("/list{_:/?}", web::get().to(handler::get_lists))
                .route("/list/{list_id}", web::get().to(handler::get_tasks))
                .route("/list/{list_id}/task", web::post().to(handler::create_task))
                .route("/list/{list_id}/task/{task_id}", web::delete().to(handler::delete_task))
                .route("/list/{list_id}/task/{task_id}", web::get().to(handler::get_one_task))
                .route("/list/{list_id}/task/{task_id}/check", web::put().to(handler::check_task))
        })
        .bind(address)?
        .run()
        .await
}
