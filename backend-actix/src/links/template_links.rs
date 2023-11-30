use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use std::{fs::File, io::{self, BufReader, BufRead}, path::Path};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct FormPageData<'a> {
    pub user_name: &'a str,
    pub form_name: &'a str,
}

pub async fn open_form_page(
    hb: web::Data<Handlebars<'_>>,
    path: web::Path<(String,)>,
) -> HttpResponse {
    let requested_form_name = &path.0;
    println!("Received form name: {}", requested_form_name);

    // Extract form name from the file
    if let Some(actual_form_name) = extract_form_name(requested_form_name).await {
        let data = FormPageData {
            user_name: &actual_form_name.clone(),
            form_name: &actual_form_name,
        };

        if let Some(body) = hb.render("form_page_template", &data).ok() {
            return HttpResponse::Ok().content_type("text/html").body(body);
        } else {
            eprintln!("Failed to render template for form: {}", requested_form_name);
        }
    } else {
        eprintln!("Failed to extract form name: {}", requested_form_name);
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

    // Print form names for debugging
    println!("Form names: {:?}", form_names);

    Ok(form_names)
}


async fn extract_user_name(form_name: &str) -> Option<String> {
    let lines = read_form_names_from_file().ok()?;
    for line in lines {
        let parts: Vec<&str> = line.rsplitn(3, ' ').map(|s| s.trim()).collect();
        if parts.len() == 3 && parts[0] == form_name.trim() {
            // Extract user_name from the line
            let user_name = parts[1].to_string();
            return Some(user_name);
        }
    }
    None
}

async fn extract_form_name(requested_form_name: &str) -> Option<String> {
    let lines = read_form_names_from_file().ok()?;
    for line in lines {
        let parts: Vec<&str> = line.rsplitn(3, ' ').map(|s| s.trim()).collect();
        if parts.len() == 3 && parts[2] == requested_form_name {
            return Some(parts[2].to_string());
        }
    }
    None
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
