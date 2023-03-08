use actix_files::Files;
use actix_web::{HttpServer, web, App};

mod routes;
use routes::converter::converter;
use routes::open::open;
use routes::viewer::viewer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(open))
            .service(converter)
            .service(viewer)
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind("localhost:8080")
    .unwrap()
    .run()
    .await
}