// src/handlers.rs: 
use crate::{
    schema::{CreateMessageSchema, FilterAllMessagesOptions, FilterOnFormOptions},
    AppState,
};

use crate::handler::models::form_models::{
    RandomStringModel,
    AnsweredCounterModel,
    PublishingControlModel,
};

use serde::Deserialize;
use chrono::{ Utc, TimeZone};

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;

#[get("/live")]
async fn form_live_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Form system CRUD API with Rust, SQLX, MySQL, and Actix Web";

    HttpResponse::Ok().json(json!({"status": "success","form_message": MESSAGE}))
}

#[get("/messages")]
pub async fn every_message_handler(
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
async fn add_message_handler(
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

//Users api:

#[post("/user/info/by_username")]
pub async fn get_user_by_username(
    account: web::Json<AuthUserRequestModelUsername>,
    data: web::Data<AppState>,
) -> impl Responder {
    // Destructure the info into individual variables
    let username = &account.username;
    let password = &account.password;
    println!("Username: {}", username);
    println!("Password: {}", password);

    // Hash the password using a proper hashing function (e.g., argon2)
    let password_hash = hash_password(password);

    let users: Result<Vec<UserModel>, sqlx::Error> = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE username = ? AND password_hash = ?",
        username,
        &password_hash
    )
    .fetch_all(&data.db)
    .await;

    match users {
        Ok(users) => {
            let user_responses = users
                .into_iter()
                .map(|user| {
                    // Convert Option<chrono::DateTime<Utc>> to String
                    let registration_date_string = user
                        .registration_date
                        .map(|date| Utc.from_utc_datetime(&date.naive_utc()).format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_default();

                    UserModelResponse {
                        id: user.id,
                        username: user.username,
                        email: user.email,
                        registration_date: Some(registration_date_string), // Use String here
                    }
                })
                .collect::<Vec<UserModelResponse>>();

            let json_response = serde_json::json!({
                "status": "success",
                "results": user_responses.len(),
                "users": user_responses
            });

            HttpResponse::Ok().json(json_response)
        }
        Err(err) => {
            let json_response = serde_json::json!({
                "status": "error",
                "message": err.to_string()
            });

            HttpResponse::InternalServerError().json(json_response)
        }
    }
}

#[post("/user/info/by_email")]
pub async fn get_user_by_email(
    account: web::Json<AuthUserRequestModelMail>,
    data: web::Data<AppState>,
) -> impl Responder {
    // Destructure the info into individual variables
    let email = &account.email;
    let password = &account.password;
    println!("Email:    {}", email);
    println!("Password: {}", password);

    // Hash the password using a proper hashing function (e.g., argon2)
    let password_hash = hash_password(password);

    let users: Result<Vec<UserModel>, sqlx::Error> = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE email = ? AND password_hash = ?",
        email,
        &password_hash
    )
    .fetch_all(&data.db)
    .await;

    match users {
        Ok(users) => {
            let user_responses = users
                .into_iter()
                .map(|user| {
                    // Convert Option<chrono::DateTime<Utc>> to String
                    let registration_date_string = user
                        .registration_date
                        .map(|date| Utc.from_utc_datetime(&date.naive_utc()).format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_default();

                    UserModelResponse {
                        id: user.id,
                        username: user.username,
                        email: user.email,
                        registration_date: Some(registration_date_string),
                    }
                })
                .collect::<Vec<UserModelResponse>>();

            let json_response = serde_json::json!({
                "status": "success",
                "results": user_responses.len(),
                "users": user_responses
            });

            HttpResponse::Ok().json(json_response)
        }
        Err(err) => {
            let json_response = serde_json::json!({
                "status": "error",
                "message": err.to_string()
            });

            HttpResponse::InternalServerError().json(json_response)
        }
    }
}


// Helper function to hash the password using a secure hash function
fn hash_password(password: &str) -> String {
    // Implement your password hashing logic here, for example using argon2
    // This is a placeholder, you should use a proper password hashing library
    // Don't forget to handle errors appropriately
    format!("{}", password)
}

#[post("/user/register/")]
pub async fn add_user(
    new_account: web::Json<UserAddRequestModel>,
    data: web::Data<AppState>,
) -> impl Responder {

    let email = &new_account.email;
    let username = &new_account.username;
    let password = &new_account.password;

    println!("Username: {}", new_account.username);
    println!("Password: {}", new_account.password);
    println!("Email: {}", new_account.email);
    
    let password_hash = password;

    // Check if the user with the same username exists
    let existing_user: Result<Option<UserModel>, sqlx::Error> = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE username = ?",
        username
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|err| {
        eprintln!("Database query failed: {}", err);
        err
    });

    match existing_user {
        Ok(Some(_)) => {
            // User with the same username already exists
            let json_response = serde_json::json!({
                "status": "error",
                "message": "User with the same username already exists"
            });

            HttpResponse::Conflict().json(json_response)
        }
        Ok(None) => {
            // Check if the user with the same email exists
            let existing_email: Result<Option<UserModel>, sqlx::Error> = sqlx::query_as!(
                UserModel,
                "SELECT * FROM users WHERE email = ?",
                email
            )
            .fetch_optional(&data.db)
            .await
            .map_err(|err| {
                eprintln!("Database query failed: {}", err);
                err
            });

            match existing_email {
                Ok(Some(_)) => {
                    // User with the same email already exists
                    let json_response = serde_json::json!({
                        "status": "error",
                        "message": "User with the same email already exists"
                    });

                    HttpResponse::Conflict().json(json_response)
                }
                Ok(None) => {
            
                    // Proceed to add the new user
                    let result = sqlx::query!(
                        "INSERT INTO users (username, email, password_hash) VALUES ( ?, ?, ?)",
                        username,
                        email,
                        password_hash
                    )
                    .execute(&data.db)
                    .await;

                    match result {
                        Ok(_) => {
                            let json_response = serde_json::json!({
                                "status": "success",
                                "message": "User added successfully"
                            });

                            HttpResponse::Ok().json(json_response)
                        }
                        Err(err) => {
                            let json_response = serde_json::json!({
                                "status": "error",
                                "message": err.to_string() // Use to_string() method directly
                            });

                            HttpResponse::InternalServerError().json(json_response)
                        }
                    }
                }
                Err(err) => {
                    // Handle the database error for email check
                    let json_response = serde_json::json!({
                        "status": "error",
                        "message": err.to_string()
                    });

                    HttpResponse::InternalServerError().json(json_response)
                }
            }
        }
        Err(err) => {
            // Handle the database error for username check
            let json_response = serde_json::json!({
                "status": "error",
                "message": err.to_string()
            });

            HttpResponse::InternalServerError().json(json_response)
        }
    }
}


pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")

        .service(form_live_checker_handler)
        .service(every_message_handler)
        .service(add_message_handler)
        .service(get_user_by_username)
        .service(get_user_by_email)
        .service(add_user)
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