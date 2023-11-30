use crate::{
    schema::{CreateMessageSchema, FilterAllMessagesOptions, FilterOnFormOptions},
    handler::models::user_models::{
            UserModel,
            UserSecurityModelQuestion,
            UserSecurityModelTelephoneNumber,
            UserSecurityModelSavingMail,
            UserModelSocials,
            UserModelResponse,
            UserAddRequestModel,
            AuthUserRequestModelUsername,
            AuthUserRequestModelMail,
        },
    AppState,
};

use serde::Deserialize;
use chrono::{ Utc, TimeZone};

use actix_web::{delete, get, put ,patch, post, web, HttpResponse, Responder};
use serde_json::json;
use sqlx::{MySql, FromRow};

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

//Users api:

#[post("/user/get_info/by_username")]
pub async fn get_user_by_username(
    account: web::Json<AuthUserRequestModelUsername>,
    data: web::Data<AppState>,
) -> impl Responder {
    // Destructure the info into individual variables
    let username: &String = &account.username;
    let password: &String = &account.password;
    println!("Username: {}", username);
    println!("Password: {}", password);

    // Hash the password using a proper hashing function (e.g., argon2)
    let password_hashed: String = hash_password(password);

    let sql_query = format!(
        "SELECT * FROM users WHERE username = ? AND password_hash = ?",
    );

    let users: Result<Vec<UserModel>, _> = sqlx::query_as::<MySql, UserModel>(&sql_query)
        .bind(username as &String)
        .bind(password_hashed as String)
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

                        let last_login_date_string = user
                        .last_login_date
                        .map(|date| Utc.from_utc_datetime(&date.naive_utc()).format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_default();


                    UserModelResponse {
                        user_id: user.id,
                        username: user.username,
                        email: user.email,
                        last_login_date: Some(last_login_date_string),
                        registration_date: Some(registration_date_string)

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

#[post("/user/get_info/by_email")]
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
    let password_hashed: String = hash_password(password);

    let sql_query = format!(
        "SELECT * FROM users WHERE email = ? AND password_hash = ?",
    );

    let users: Result<Vec<UserModel>, _> = sqlx::query_as::<MySql, UserModel>(&sql_query)
        .bind(email as &String)
        .bind(password_hashed as String)
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

                        let last_login_date_string = user
                        .last_login_date
                        .map(|date| Utc.from_utc_datetime(&date.naive_utc()).format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_default();


                    UserModelResponse {
                        user_id: user.id,
                        username: user.username,
                        email: user.email,
                        last_login_date: Some(last_login_date_string),
                        registration_date: Some(registration_date_string)

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


#[derive(serde::Deserialize)]
struct ChangePasswordRequest {
    username: String,
    current_password: String,
    new_password: String,
}

#[put("/user/change-password/")]
pub async fn change_password(
    payload: web::Json<ChangePasswordRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let username = &payload.username;
    let current_password = &payload.current_password;
    let new_password = &payload.new_password;

    // Check if the user with the specified username and current password exists
    let existing_user: Result<Option<UserModel>, sqlx::Error> = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE username = ? AND password_hash = ?",
        username,
        current_password
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|err| {
        eprintln!("Database query failed: {}", err);
        err
    });

    match existing_user {
        Ok(Some(_)) => {
            // User with the specified username and current password exists, proceed to update password
            let result = sqlx::query!(
                "UPDATE users SET password_hash = ? WHERE username = ?",
                new_password,
                username
            )
            .execute(&data.db)
            .await;

            match result {
                Ok(_) => {
                    let json_response = serde_json::json!({
                        "status": "success",
                        "message": "Password updated successfully"
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
        Ok(None) => {
            // User with the specified username and current password does not exist
            let json_response = serde_json::json!({
                "status": "error",
                "message": "Invalid username or current password"
            });

            HttpResponse::Unauthorized().json(json_response)
        }
        Err(err) => {
            // Handle the database error for username and password check
            let json_response = serde_json::json!({
                "status": "error",
                "message": err.to_string()
            });

            HttpResponse::InternalServerError().json(json_response)
        }
    }
}


#[derive(serde::Deserialize)]
pub struct DeleteUserRequest {
   pub username: String,
   pub password: String,
}

#[delete("/user/delete/")]
pub async fn delete_user(
    payload: web::Json<DeleteUserRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    let username = &payload.username;
    let password = &payload.password;

    // Check if the user with the specified username and password exists
    let existing_user: Result<Option<UserModel>, sqlx::Error> = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE username = ? AND password_hash = ?",
        username,
        password
    )
    .fetch_optional(&data.db)
    .await
    .map_err(|err| {
        eprintln!("Database query failed: {}", err);
        err
    });

    match existing_user {
        Ok(Some(_)) => {
            // User with the specified username and password exists, proceed to delete
            let result = sqlx::query!(
                "DELETE FROM users WHERE username = ?",
                username
            )
            .execute(&data.db)
            .await;

            match result {
                Ok(_) => {
                    let json_response = serde_json::json!({
                        "status": "success",
                        "message": "User deleted successfully"
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
        Ok(None) => {
            // User with the specified username and password does not exist
            let json_response = serde_json::json!({
                "status": "error",
                "message": "Invalid username or password"
            });

            HttpResponse::Unauthorized().json(json_response)
        }
        Err(err) => {
            // Handle the database error for username and password check
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

pub fn user_handler_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(get_user_by_username)
        .service(get_user_by_email)
        .service(delete_user)
        .service(change_password)
        .service(add_user);
    conf.service(scope);
}