// src/main.rs
use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, HttpRequest};
use links::greet_user; // Import the greet_user function from links module

mod links;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(links::config)
            .service(web::scope("/api").configure(links::scoped_config))
            .route("/", web::get().to(|| async { actix_web::HttpResponse::Ok().body("Home Page") }))
            .route("/hello/{name}", web::get().to(greet_user)) // Use the greet_user function
            .service(
                web::resource("/index.html")
                    .route(web::get().to(|_req: HttpRequest| async {
                        let file = NamedFile::open("static/index.html");
                        match file {
                            Ok(file) => Ok(file),
                            Err(err) => Err(actix_web::error::ErrorInternalServerError(err)),
                        }
                    }),
                )
            )
            .service(
                actix_files::Files::new("/static", "static").show_files_listing()
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
