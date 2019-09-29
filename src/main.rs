#[macro_use]
extern crate serde_derive;
extern crate actix_web;
extern crate derive_more;
extern crate futures;
extern crate oidc;
extern crate reqwest;
extern crate serde;

use actix_web::{middleware, web, App, HttpResponse, HttpServer};

mod errors;
mod handler;

fn index() -> web::HttpResponse {
    HttpResponse::Ok().json("Hello World")
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .data(web::JsonConfig::default().limit(4096))
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/auth").route(web::get().to_async(handler::auth::auth)))
            .service(web::resource("/callback").route(web::get().to(handler::auth::callback)))
    })
    .bind("0.0.0.0:3000")?
    .run()
}
