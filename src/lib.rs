use actix_web::dev::Server;
use actix_web::{web, App, HttpServer, Responder};
use std::net::TcpListener;

pub mod routes;

use routes::health_check;
use routes::subscribe;

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscribe", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
