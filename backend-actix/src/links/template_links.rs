// /links/template_links.rs:

use serde::Deserialize;
use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use serde_json::json;
use std::{path::Path, fs::File, io::{BufReader, BufRead, self}};


#[derive(Deserialize, Debug)]
struct FormFilterOptions {
    pub form_name: String,
}

pub async fn open_form_page(
    hb: web::Data<Handlebars<'_>>,
    path: web::Path<(String,)>,
) -> HttpResponse {
    let form_name = path.0.clone();

    // Geçerli form adlarını kontrol et
    if valid_form_name(&form_name) {
        // Handlebars şablonunu oluşturacak veriyi hazırlayın
        let data = json!({
            "form_name": form_name,
        });

        // Handlebars ile şablonu render edin
        let template_name = "form_page_template";
        if let Some(body) = hb.render(template_name, &data).ok() {
            // HTTP yanıtını oluşturun
            return HttpResponse::Ok().content_type("text/html").body(body);
        }
    }

    // Geçersiz form adı veya şablon oluşturulamadıysa 404 hatası döndür
    HttpResponse::NotFound().finish()
}

// form_names.txt dosyasından form adlarını kontrol eden bir yardımcı fonksiyon
fn valid_form_name(form_name: &str) -> bool {
    match read_form_names_from_file() {
        Ok(form_names) => form_names.contains(&form_name.to_string()),
        Err(_) => false,
    }
}

// form_names.txt dosyasından form adlarını okuyan bir yardımcı fonksiyon
fn read_form_names_from_file() -> Result<Vec<String>, io::Error> {
    let file_path = "form_names.txt";
    let path = Path::new(&file_path);

    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let form_names: Vec<String> = reader.lines().filter_map(|line| line.ok()).collect();

    Ok(form_names)
}

pub fn form_page_template_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/form/{form_name}")
            .to(open_form_page),
    );
}
