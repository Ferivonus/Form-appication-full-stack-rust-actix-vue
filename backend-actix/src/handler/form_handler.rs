
// src/form_handler.rs: 
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

#[get("/form/messages")]
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


#[get("/form/messages/{form_name}")]
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

    if !valid_form_name(data.clone(), &form_name_of_link).await {
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

async fn valid_form_name(data: web::Data<AppState>, form_name: &String) -> bool {
    let searching_data_query = format!(
        "SELECT EXISTS(SELECT 1 FROM form_pages WHERE form_name = '{}') AS exists",
        &form_name
    );

    let result: Result<bool, _> = sqlx::query_scalar(&searching_data_query)
        .fetch_one(&data.db)
        .await;

    match result {
        Ok(exists) => exists,
        Err(err) => {
            eprintln!("Error checking if form name exists: {}", err);
            false
        }
    }
}

#[post("/form/messages/")]
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




#[delete("/form/delete-message")]
pub async fn delete_message_handler(
    opts: web::Json<FilterOnFormOptions>,
    data: web::Data<AppState>,
    path: web::Path<(String,)>,
) -> impl Responder {

    // Use the random_string_identifier from the query parameters or request body
    //let random_string_identifier = opts.random_string_identifier.clone().unwrap_or_default();

    // Your logic to delete the message based on random_string_identifier goes here
    // Use SQL DELETE statement or your preferred method

    let json_response = serde_json::json!({
        "status": "success",
        "message": "Message deleted successfully"
    });

    HttpResponse::Ok().json(json_response)
}


#[derive(Debug, Deserialize)]
pub struct AnswerMessageSchema {
    pub random_string_identifier: String,
    pub user_token: String,
    // Add other fields as needed for your answer schema
}    
    
#[derive(Debug, Deserialize)]
pub struct DelateMessageSchema {
    pub random_string_identifier: String,
    pub user_token: String,
    // Add other fields as needed for your answer schema
}

#[post("/messages/answer")]
async fn answer_to_message_handler(
    data: web::Data<AppState>,
    body: web::Json<AnswerMessageSchema>,
) -> impl Responder {
    // Your logic to handle answering a message goes here
    // You can access the random_string_identifier using body.random_string_identifier
    // Add necessary database queries and error handling

    // Example response
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "Message answered successfully"
    }))
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
    .service(form_message_list_by_form_name_handler)
    .service(answer_to_message_handler)
    .service(delete_message_handler)
    .service(multiply);
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