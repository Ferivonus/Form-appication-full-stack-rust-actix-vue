// main.rs:
use actix_files::NamedFile;
use actix_web::{http::header, web, App, HttpServer, HttpRequest};
use handlebars::Handlebars;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use dotenv::dotenv;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};

mod schema;
mod handler;
mod links;

pub struct AppState {
    db: MySqlPool,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };


    println!("🚀 ractix back-end system started successfully");


    let mut book_handlebars = Handlebars::new();
    book_handlebars
        .register_template_file("bookpage_template", "static/templates/bookpage_template.html")
        .unwrap();
    let book_handlebars_ref = web::Data::new(book_handlebars);
    
    
    let mut form_handlebars = Handlebars::new();
    form_handlebars
        .register_template_file("form_page_template", "static/templates/formpage_template.html")
        .unwrap();
    let form_handlebars_ref2 = web::Data::new(form_handlebars);
   
    HttpServer::new(move || {

        let cors = Cors::default()
            //.allowed_origin("http://localhost:8080")
            .allowed_origin("http://localhost:8080")
            .allowed_methods(vec!["GET", "POST", "PATCH", "DELETE"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();

        App::new()
            .app_data(book_handlebars_ref.clone())
            .app_data(form_handlebars_ref2.clone())
            .app_data(web::Data::new(AppState { db: pool.clone() }))
            .configure(handler::handler::config)
            .wrap(cors)
            .wrap(Logger::default())
            .route("/bookpage", web::get().to(links::book_page))
            .route("/", web::get().to(|_req: HttpRequest| async {
                let file = NamedFile::open("static/index.html");
                match file {
                    Ok(file) => Ok(file),
                    Err(err) => Err(actix_web::error::ErrorInternalServerError(err)),
                }
            }))
            .service(
                web::resource("/user/{user_link}")
                    .route(web::get().to(links::serve_user_page)),
            )
            .service(
                actix_files::Files::new("/static", "static").show_files_listing()
            )
            .configure(links::template_links::form_page_template_config)

    })
    .bind("127.0.0.1:8080")?
    // .bind("127.0.0.1:80")? masal için tor (These lines are commented out)
    // .bind("127.0.0.1:8080")? BİLGİSAYAR için tor (These lines are commented out)
    .run()
    .await
}
