use crate::embed::handle_embedded_file;
use actix_web::{get, web, HttpResponse};

pub async fn home() -> HttpResponse {
    handle_embedded_file("index.html")
}

#[get("/")]
pub async fn index() -> HttpResponse {
    handle_embedded_file("index.html")
}

#[get("/favicon.ico")]
pub async fn favicon() -> HttpResponse {
    handle_embedded_file("favicon.ico")
}

#[get("/assets/{name}")]
pub async fn assets(name: web::Path<String>) -> HttpResponse {
    handle_embedded_file(format!("assets/{}", name).as_str())
}
