use actix_files::NamedFile;
use actix_web::{web, App, HttpServer, HttpRequest, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;

mod links;

pub async fn book_page(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "project_name": "Book Store",
        "books": [
            {
                "name": "Harry Potter",
                "author": "J K Rowling",
                "image_path": "/static/image/download.jpeg",
            },
            {
                "name": "Lord of the Rings",
                "author": "Tolkien",
                "image_path": "/static/image/lord_of.jpeg",
            },
            {
                "name": "Americanah",
                "author": "Chimamanda Ngozi Adichie",
                "image_path": "/static/image/americanah.jpeg",
            },
            {
                "name": "Elon Musk",
                "author": "Unknown",
                "image_path": "/static/image/elon.jpeg",
            },
        ],
    });

    let template_name = "bookpage_template"; // Name of your template file without the extension
    let body = hb.render(template_name, &data).unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("bookpage_template", "static/templates/bookpage_template.html")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .configure(links::config)
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("Home Page") }))
            .route("/hello/{name}", web::get().to(links::greet_user))
            .route("/bookpage", web::get().to(links::book_page))
            .service(
                web::resource("/index.html")
                    .route(web::get().to(|_req: HttpRequest| async {
                        let file = NamedFile::open("static/index.html");
                        match file {
                            Ok(file) => Ok(file),
                            Err(err) => Err(actix_web::error::ErrorInternalServerError(err)),
                        }
                    }),
                ))
            .service(
                actix_files::Files::new("/static", "static").show_files_listing()
            )
            .service(web::scope("/api").configure(links::scoped_config))
    })
    .bind("127.0.0.1:8080")?
    // .bind("127.0.0.1:80")? masal için tor (These lines are commented out)
    // .bind("127.0.0.1:8080")? BİLGİSAYAR için tor (These lines are commented out)
    .run()
    .await
}
