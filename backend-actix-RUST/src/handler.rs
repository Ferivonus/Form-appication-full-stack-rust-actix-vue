// src/handlers.rs: 
use crate::{
    model::{FormMessageModel, FormMessageModelResponse, UserModel, UserModelResponse},
    schema::{CreateMessageSchema, FilterAllMessagesOptions, FilterOnFormOptions, UpdateMessageSchema },
    AppState,
};

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;

#[get("/api_info")]
async fn form_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Form system CRUD API with Rust, SQLX, MySQL, and Actix Web";

    HttpResponse::Ok().json(json!({"status": "success","form_message": MESSAGE}))
}

#[get("/messages")]
pub async fn full_form_message_list_handler(
    opts: web::Query<FilterAllMessagesOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let messages: Vec<FormMessageModel> = sqlx::query_as!(
        FormMessageModel,
        r#"SELECT * FROM form_messages ORDER by id LIMIT ? OFFSET ?"#,
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let note_responses = messages
        .into_iter()
        .map(|note| filter_db_record(&note))
        .collect::<Vec<FormMessageModelResponse>>();

    let json_response = serde_json::json!({
        "status": "success",
        "results": note_responses.len(),
        "messages": note_responses
    });
    HttpResponse::Ok().json(json_response)
}


#[get("/messages/{form_name}")]
pub async fn form_message_list_by_form_name_handler(
    opts: web::Query<FilterOnFormOptions>,
    data: web::Data<AppState>,
    path: web::Path<(String,)>,
) -> impl Responder {
    let form_name_of_link = path.0.clone();
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    // If form_title is present and not equal to form_name_of_link, return a bad request
    if let Some(ref form_title) = opts.form_title {
        if form_name_of_link != *form_title {
            return HttpResponse::BadRequest().json(json!({
                "status": "fail",
                "message": "form_name_of_link and form_title must be the same"
            }));
        }
    }


    let messages: Vec<FormMessageModel> = sqlx::query_as!(
        FormMessageModel,
        r#"SELECT * FROM form_messages WHERE form_title = ? AND published = true ORDER by id LIMIT ? OFFSET ?"#,
        form_name_of_link,
        limit as i32,
        offset as i32
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let note_responses = messages
        .into_iter()
        .map(|note| filter_db_record(&note))
        .collect::<Vec<FormMessageModelResponse>>();

    let json_response = serde_json::json!({
        "status": "success",
        "results": note_responses.len(),
        "messages": note_responses
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/messages/")]
async fn create_message_handler(
    body: web::Json<CreateMessageSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let user_id = uuid::Uuid::new_v4().to_string();
    let query_result =
        sqlx::query(r#"INSERT INTO form_messages (id,title,content,form_title,published) VALUES (?, ?, ?, ?, ?)"#)
            .bind(user_id.clone())
            .bind(body.title.to_string())
            .bind(body.content.to_string())
            .bind(body.form_title.to_owned().unwrap_or_default())
            .bind(body.published.to_owned().unwrap_or_default())
            .execute(&data.db)
            .await
            .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            return HttpResponse::BadRequest().json(
            serde_json::json!({"status": "fail","form_message": "form with that title already exists"}),
        );
        }

        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","form_message": format!("{:?}", err)}));
    }

    let query_result = sqlx::query_as!(FormMessageModel, r#"SELECT * FROM form_messages WHERE id = ?"#, user_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(note) => {
            let note_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "note": filter_db_record(&note)
            })});

            return HttpResponse::Ok().json(note_response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","form_message": format!("{:?}", e)}));
        }
    }
}


fn filter_db_record(form_message: &FormMessageModel) -> FormMessageModelResponse {
    FormMessageModelResponse {
        id: form_message.id.to_owned(),
        title: form_message.title.to_owned(),
        content: form_message.content.to_owned(),
        form_title: form_message.form_title.to_owned().unwrap(),
        published: form_message.published != 0,
        createdAt: form_message.created_at.unwrap(),
        updatedAt: form_message.updated_at.unwrap(),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")

        .service(form_checker_handler)
        .service(full_form_message_list_handler)
        .service(create_message_handler)
        .service(form_message_list_by_form_name_handler);

    conf.service(scope);
}