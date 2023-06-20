use super::api::{assets, favicon, home, index};
use crate::config::{IP, PORT};
use actix_web::{dev::Server, web, App, HttpServer, Result};

pub fn http_server() -> Result<Server> {
    let server = HttpServer::new(|| {
        App::new()
            .service(index)
            .service(favicon)
            .service(assets)
            .route("/{_:.*}", web::get().to(home))
    })
    .bind((IP, PORT))?
    .run();
    Ok(server)
}
