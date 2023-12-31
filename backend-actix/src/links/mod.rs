// src/links/mod.r

use actix_files::NamedFile;
use actix_web::{web, HttpResponse, HttpRequest};
use handlebars::Handlebars;
use serde_json::json;

pub mod template_links;

pub async fn serve_user_page(req: HttpRequest) -> actix_web::Result<NamedFile> {
    // Extract the Form name from the request path
    let form_name = req.match_info().query("user_link");

    let form_file_path = "static/user pages/";
    // Manually specify the allowed Form names
    let allowed_form_names = ["login", "register"];

    // Check if the requested form name is allowed
    if allowed_form_names.contains(&form_name) {
        // Construct the file path by appending the form name and ".html" to the "form Pages" directory
        let path = format!("{}user_{}.html", form_file_path, form_name);

        println!("The path is: {}", path);
        // Attempt to open the file
        match NamedFile::open(path) {
            Ok(file) => Ok(file),
            Err(err) => Err(actix_web::error::ErrorInternalServerError(err)),
        }
    } else {
        // Serve a default HTML page for non-existing forms
        match NamedFile::open("static/user pages/user_default.html") {
            Ok(file) => Ok(file),
            Err(err) => {
                eprintln!("Error opening default file: {:?}", err);
                Err(actix_web::error::ErrorInternalServerError(err))
            }
        }
    }
}

pub async fn book_page(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "project_name": "form",
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

    let template_name = "book_page_template"; // Name of your template file without the extension
    let body = hb.render(template_name, &data).unwrap();
    HttpResponse::Ok().content_type("text/html").body(body)
}

