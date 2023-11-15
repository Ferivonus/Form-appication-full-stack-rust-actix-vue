// src/links/mod.rs

use actix_web::{web, HttpResponse, HttpRequest};
use handlebars::Handlebars;
use serde_json::json;

// Import Serializer for custom serialization

pub async fn serve_form_page(req: HttpRequest) -> actix_web::Result<actix_files::NamedFile> {
    // Extract the Form name from the request path
    let form_name = req.match_info().query("form_link");

    println!("I am trying to find: {}",form_name);
    let form_file_path = "static/Form Pages/";
    println!("I declare path ");
    // Manually specify the allowed Form names
    let allowed_form_names = ["chatting","astrology", "game", "sport", "software"];

    // Check if the requested form name is allowed
    if allowed_form_names.contains(&form_name) {
        println!("I tried to find page");
        // Construct the file path by appending the form name and ".html" to the "form Pages" directory
        let path = format!("{}{}_form.html",form_file_path, form_name);

        println!("The path is: {}",path);
        let file = actix_files::NamedFile::open(path);

        // Attempt to open the file
        match file {
            Ok(file) => Ok(file),
            Err(err) => Err(actix_web::error::ErrorInternalServerError(err)),
        }
    } else {
        // Serve a default HTML page for non-existing forms
        let default_file = actix_files::NamedFile::open("static/Form Pages/default_form.html");
        match default_file {
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