#[macro_use]
extern crate serde_derive;

mod handlers;

use actix_web::{middleware, web, App, HttpServer};
use std::{env, io};

fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let sys = actix_rt::System::new("todo-api");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/todos")
                    .route(web::get().to(handlers::todos))
                    .route(web::post().to(handlers::create)),
            )
            .service(
                web::resource("/todos/{task_id}")
                    .route(web::get().to(handlers::todo))
                    .route(web::put().to(handlers::update))
                    .route(web::delete().to(handlers::delete)),
            )
    })
    .bind("127.0.0.1:8080")?
    .start();

    sys.run()
}
