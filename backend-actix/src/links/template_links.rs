use actix_web::{web, HttpResponse};
use handlebars::Handlebars;
use sqlx::query_as;
use crate::AppState;

#[derive(sqlx::FromRow, serde::Serialize, serde::Deserialize, Debug)]
pub struct FormPageData {
    pub creator_user_name: String,
    pub form_name: String,
    pub creator_id: i32,
    pub used_time: i64,
}

pub async fn open_form_page(
    hb: web::Data<Handlebars<'_>>,
    path: web::Path<(String,)>,
    data: web::Data<AppState>,
) -> HttpResponse {
    let requested_form_name = &path.0;
    println!("Received form name: {}", requested_form_name);

    // Use a query to get data from the database
    let query_result = query_as::<_, FormPageData>(
        "SELECT creator_user_name, form_name, creator_id, used_time FROM form_pages WHERE form_name = '?'",
    )
    .bind(requested_form_name)
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(mut form_data) => {
            // Increment the used_time
            form_data.used_time += 1;

            // Update the used_time in the database
            let update_result = sqlx::query("UPDATE form_pages SET used_time = ? WHERE form_name = ?")
                .bind(form_data.used_time)
                .bind(&form_data.form_name)
                .execute(&data.db)
                .await;

            match update_result {
                Ok(_) => {
                    let data = FormPageData {
                        creator_user_name: form_data.creator_user_name,
                        form_name: form_data.form_name,
                        creator_id: form_data.creator_id,
                        used_time: form_data.used_time,
                    };

                    if let Some(body) = hb.render("form_page_template", &data).ok() {
                        return HttpResponse::Ok().content_type("text/html").body(body);
                    } else {
                        eprintln!("Failed to render template for form: {}", requested_form_name);
                    }
                }
                Err(err) => {
                    eprintln!("Error updating used_time in the database: {}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Error fetching data from the database: {}", err);
        }
    }

    HttpResponse::NotFound().finish()
}

pub fn form_page_template_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/form/{form_name}")
            .to(open_form_page),
    );
}
