use actix_web::{Responder, HttpResponse, web};
use deadpool_postgres::{Client, Pool};

use crate::repo;
use crate::model::{NewTask, Status};

pub async fn status() -> impl Responder {
    HttpResponse::Ok()
        .json(Status::ok())
}

pub async fn get_lists(pool: web::Data<Pool>) -> impl Responder {
    let client: Client = pool.get().await
        .expect("Failed to acquire DB connection");

    let result = repo::get_lists(&client).await;

    match result {
        Ok(lists) => HttpResponse::Ok().json(lists),
        Err(_) => HttpResponse::NotFound().into()
    }
}

pub async fn get_tasks(pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = pool.get().await
        .expect("Failed to acquire DB connection");

    let result = repo::get_tasks(&client, path.0).await;

    match result {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::NotFound().into()
    }
}

pub async fn get_one_task(pool: web::Data<Pool>, path: web::Path<(i32, i32)>) -> impl Responder {
    let client: Client = pool.get().await
        .expect("Failed to acquire DB connection");

    let result = repo::get_one_task(&client, path.0, path.1).await;

    match result {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(_) => HttpResponse::NotFound().into()
    }
}

pub async fn create_task(pool: web::Data<Pool>, path: web::Path<(i32,)>, json: web::Json<NewTask>) -> impl Responder {
    let client: Client = pool.get().await
        .expect("Failed to acquire DB connection");

    let result = repo::create_task(&client, path.0, &json.title).await;

    match result {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(_) => HttpResponse::NotFound().into()
    }
}

pub async fn check_task(pool: web::Data<Pool>, path: web::Path<(i32, i32)>) -> impl Responder {
    let client: Client = pool.get().await
        .expect("Failed to acquire DB connection");

    let result = repo::check_task(&client, path.0, path.1).await;

    match result {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}

pub async fn delete_task(pool: web::Data<Pool>, path: web::Path<(i32, i32)>) -> impl Responder {
    let client: Client = pool.get().await
        .expect("Failed to acquire DB connection");

    let result = repo::delete_task(&client, path.0, path.1).await;

    match result {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}
