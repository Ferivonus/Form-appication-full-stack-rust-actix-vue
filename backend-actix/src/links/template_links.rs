use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;
use std::{fs::File, io::{self, BufReader, BufRead}, path::Path};

pub async fn open_form_page(
    hb: web::Data<Handlebars<'_>>,
    path: web::Path<(String,)>,
) -> HttpResponse {
    let form_name = &path.0;
    println!("Received form name: {}", form_name);

    if valid_form_name(form_name) {
        println!("Valid form name: {}", form_name);

        let data = json!({
            "form_name": form_name,
        });

        if let Some(body) = hb.render("form_page_template", &data).ok() {
            return HttpResponse::Ok().content_type("text/html").body(body);
        } else {
            eprintln!("Failed to render template for form: {}", form_name);
        }
    } else {
        eprintln!("Invalid form name: {}", form_name);
    }

    HttpResponse::NotFound().finish()
}

fn valid_form_name(form_name: &str) -> bool {
    match read_form_names_from_file() {
        Ok(form_names) => form_names.contains(&form_name.to_string()),
        Err(err) => {
            eprintln!("Error reading form names from file {}: {}", get_file_path(), err);
            false
        }
    }
}

fn read_form_names_from_file() -> Result<Vec<String>, io::Error> {
    let file_path = get_file_path();
    let path = Path::new(&file_path);

    let file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file {}: {}", file_path, err);
            return Err(err);
        }
    };

    let reader = BufReader::new(file);

    let form_names: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    Ok(form_names)
}

fn get_file_path() -> &'static str {
    "form_names.txt"
}

pub fn form_page_template_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/form/{form_name}")
            .to(open_form_page),
    );
}
