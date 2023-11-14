// src/handlers.rs: 
use crate::{
    model::{FormMessageModel, FormMessageModelResponse, UserModel, UserModelResponse},
    schema::{CreateMessageSchema, FilterOptions, UpdateMessageSchema },
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
pub async fn message_list_handler(
    opts: web::Query<FilterOptions>,
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
            serde_json::json!({"status": "fail","form_message": "Note with that title already exists"}),
        );
        }

        return HttpResponse::InternalServerError()
            .json(serde_json::json!({"status": "error","form_message": format!("{:?}", err)}));
    }

    println!("🚀 messages part worked without id");


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

fn filter_db_record(note: &FormMessageModel) -> FormMessageModelResponse {
    FormMessageModelResponse {
        id: note.id.to_owned(),
        title: note.title.to_owned(),
        content: note.content.to_owned(),
        form_title: note.form_title.to_owned().unwrap(),
        published: note.published != 0,
        createdAt: note.created_at.unwrap(),
        updatedAt: note.updated_at.unwrap(),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")

        .service(form_checker_handler)
        .service(message_list_handler)
        .service(create_message_handler);
    conf.service(scope);
}