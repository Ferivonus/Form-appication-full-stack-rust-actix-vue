// src/handlers.rs: 
use crate::AppState;


use serde::Deserialize;
use chrono::{ Utc, TimeZone};

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;
use sqlx::{MySql, FromRow};

#[get("/live")]
async fn form_live_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Form system CRUD API with Rust, SQLX, MySQL, and Actix Web";

    HttpResponse::Ok().json(json!({"status": "success","form_message": MESSAGE}))
}

#[get("/anan")]
async fn anan_handler() -> impl Responder {
    const MESSAGE: &str = "Döne dolaşa buraya geldin, tebrikler, Ama ananın amı.";

    HttpResponse::Ok().json(json!({"status": "success","form_message": MESSAGE}))
}


pub fn main_handler_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")

        .service(anan_handler)
        .service(form_live_checker_handler)
        .service(multiply);

    conf.service(scope);
}

// web query example:

#[derive(Debug, Deserialize)]
pub struct Nums {
    first: u64,
    second: u64,
}

//    /multiply?first=5&second=2
#[get("/multiply")]
pub async fn multiply(nums: web::Query<Nums>) -> impl Responder {
    format!("Result: {}!", nums.first * nums.second)
}