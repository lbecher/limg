use actix_web::{HttpResponse, Responder};
use std::fs;

pub async fn open() -> impl Responder {
    let html = fs::read_to_string("static/html/open.html").unwrap();
    HttpResponse::Ok().body(html)
}