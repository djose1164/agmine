use actix_web::{get, middleware, web, App, HttpResponse, HttpServer, Responder};

mod models;
mod repository;
mod routes;

use repository::database::establish_connection;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("<h1>Welcome to Agmine</h1>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running at: http://127.0.0.1:8080");

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(
                establish_connection().expect("Error while setting app data"),
            ))
            .wrap(middleware::Logger::default())
            .service(index)
            .service(routes::users::index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
