
// src/handlers.rs: 
use crate::{
    schema::{CreateMessageSchema, FilterAllMessagesOptions, FilterOnFormOptions},
    handler::models::form_models::{
            RandomStringModel,
            AnsweredCounterModel,
            PublishingControlModel,
            MessageInfoModel,
            HasImageInfoModel,
            ImageCounterModel,
            ImageInformationModel,
            ImageTimeInfoModel,
            ImageHowManyTimesAnsweredModel,
            ImageLikeDislikeFunnyModel,
            LikeDislikeInformationModel,
            MessageTimeInfoModel,
            AnsweredToNodeModel,
            AnsweredMessagesInfoModel,
            FormMessageModel, 
            FormMessageModelResponse,
            MessageInfoModelResponse,
        },
    AppState,
};


use serde::Deserialize;
use chrono::{ Utc, TimeZone};

use actix_web::{delete, get, put ,patch, post, web, HttpResponse, Responder};
use serde_json::json;
use sqlx::{MySql, FromRow};

#[get("/messages")]
pub async fn every_message_handler(
    opts: web::Query<FilterAllMessagesOptions>,
    data: web::Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;
    const FORM_NAME_OF_LINK : &str= "anime";

    let query = format!(
        r#"SELECT * FROM {}_form_messages_message_info ORDER BY sender_user_id LIMIT ? OFFSET ?"#,
        FORM_NAME_OF_LINK
    );

    let messages: Vec<MessageInfoModel> = sqlx::query_as::<MySql, MessageInfoModel>(&query)
    .bind(limit as i32)
    .bind(offset as i32)
    .fetch_all(&data.db)
    .await
    .unwrap();

    let note_responses = messages
        .into_iter()
        .map(|note| filter_db_record(&note))
        .collect::<Vec<MessageInfoModelResponse>>();
    

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
    let limit_count = opts.limit.unwrap_or(10);
    let offset_count = (opts.page.unwrap_or(1) - 1) * limit_count;

    // If form_title is present and not equal to form_name_of_link, return a bad request
    if let Some(ref form_title) = opts.form_title {
        if form_name_of_link != *form_title {
            return HttpResponse::BadRequest().json(json!({
                "status": "fail",
                "message": "form_name_of_link and form_title must be the same"
            }));
        }
    }

    if !valid_form_name(&form_name_of_link) {
        eprintln!("Invalid form name: {}", form_name_of_link);
        return HttpResponse::NotFound().finish();
    }

    let sql_query = format!(
        "SELECT * FROM {}_form_messages_message_info WHERE published = true ORDER BY id LIMIT ? OFFSET ?",
        form_name_of_link
    );

    match sqlx::query_as::<MySql, MessageInfoModel>(&sql_query)
        .bind(limit_count as i32)
        .bind(offset_count as i32)
        .fetch_all(&data.db)
        .await
    {
        Ok(messages) => {
            let note_responses = messages
                .into_iter()
                .map(|note| filter_db_record(&note))
                .collect::<Vec<MessageInfoModelResponse>>();

            let json_response = serde_json::json!({
                "status": "success",
                "results": note_responses.len(),
                "messages": note_responses
            });

            HttpResponse::Ok().json(json_response)
        }
        Err(err) => {
            eprintln!("Error retrieving messages: {}", err);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/messages/")]
async fn add_message_handler(
    body: web::Json<CreateMessageSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let form_title = body.form_title.to_owned().unwrap_or_default();
    let sql_query = format!(
        "INSERT INTO {}_form_messages_message_info (id,title,content,form_title,published) VALUES (?, ?, ?, ?, ?)",
        &form_title
    );

    let query_result = sqlx::query(&sql_query)
        .bind(&body.user_id)
        .bind(&body.title)
        .bind(&body.content)
        .bind(&form_title)
        .bind(body.published.unwrap_or_default())
        .execute(&data.db)
        .await
        .map_err(|err| err.to_string());

    match query_result {
        Ok(_) => {
            let getting_data_query = format!(
                "SELECT * FROM {}_form_messages WHERE id = ?",
                &form_title
            );

            let query_result: Result<MessageInfoModel, _> = sqlx::query_as::<MySql, MessageInfoModel>(&getting_data_query)
                .bind(&body.user_id)
                .fetch_one(&data.db)
                .await;

            match query_result {
                Ok(note) => {
                    let note_response = serde_json::json!({
                        "status": "success",
                        "data": {
                            "note": filter_db_record(&note)
                        }
                    });
                    HttpResponse::Ok().json(note_response)
                }
                Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({
                    "status": "error",
                    "message": format!("Error retrieving data: {:?}", e)
                })),
            }
        }
        Err(err) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("Error executing query: {:?}", err)
            }))
        }
    }
}




fn filter_db_record(record: &MessageInfoModel) -> MessageInfoModelResponse {
    MessageInfoModelResponse {
            random_string_identifier: record.random_string_identifier.clone(),
            sender_user_id: record.sender_user_id,
            title: record.title.clone(),
            content: record.content.clone(),
    }
}

pub fn form_handler_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")

        .service(every_message_handler)
        .service(add_message_handler)
        .service(multiply)
        .service(form_message_list_by_form_name_handler);

    conf.service(scope);
}




// web query example:

#[derive(Debug, Deserialize)]
pub struct Nums {
    first: u64,
    second: u64,
}

//    /multiply?first=5&second=2
#[get("/multiply")]
pub async fn multiply(nums: web::Query<Nums>) -> impl Responder {
    format!("Result: {}!", nums.first * nums.second)
}

use std::{fs::File, io::{self, BufReader, BufRead}, path::Path};

fn valid_form_name(form_name: &str) -> bool {
    match read_form_names_from_file() {
        Ok(form_names) => form_names.contains(&form_name.to_string()),
        Err(err) => {
            eprintln!("Error reading form names from file: {}", err);
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