
// src/handlers.rs: 
use crate::{
    model::{FormMessageModel, FormMessageModelResponse, UserModel, UserModelResponse,UserAddRequestModel,AuthUserRequestModel},
    schema::{CreateMessageSchema, FilterAllMessagesOptions, FilterOnFormOptions,FilterAllUserOptions },
    AppState,
};



use actix_web::body::BoxBody;
use serde::{Deserialize};
use crate::HttpRequest;
use crate::header::ContentType;


use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;
use serde::{Serialize};

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


//Users:


#[get("/users/check/{username_fromuser}/{password_fromuser}")]
pub async fn two_line_get_users(
    path: web::Path<(String, String)>,
    data: web::Data<AppState>,
) -> impl Responder {
    let (username_fromuser, password_fromuser) = path.into_inner();

    println!("Username: {}", username_fromuser);
    println!("Password: {}", password_fromuser);


    //hash functions:
    let password_hash = password_fromuser;


    let users: Result<Vec<UserModel>, sqlx::Error> = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE username = ? AND password_hash = ?",
        username_fromuser,
        password_hash
    )
    .fetch_all(&data.db)
    .await;

    match users {
        Ok(users) => {
            let user_responses = users
                .into_iter()
                .map(|user| {
                    // Convert Option<chrono::DateTime<Utc>> to Option<String>
                    let registration_date = user.registration_date.unwrap();
                    
                    UserModelResponse {
                        id: user.id,
                        username: user.username,
                        email: user.email,
                        registration_date: Some(registration_date),
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
                "message": err.to_string() // Use to_string() method directly
            });

            HttpResponse::InternalServerError().json(json_response)
        }
    }
}

// example of use: http://localhost:8080/api/users/check?username=john_doe&password=hashed_password_123
#[get("/users/check")]
pub async fn one_line_get_users(
    account: web::Query<AuthUserRequestModel>,
    data: web::Data<AppState>,
) -> impl Responder {
    // Destructure the info into individual variables
    let username = &account.username;
    let password = &account.password;
    println!("Username: {}", username);
    println!("Password: {}", password);

    //hash functions:
    let password_hash = password;

    let users: Result<Vec<UserModel>, sqlx::Error> = sqlx::query_as!(
        UserModel,
        "SELECT * FROM users WHERE username = ? AND password_hash = ?",
        username,
        password_hash
    )
    .fetch_all(&data.db)
    .await;

    match users {
        Ok(users) => {
            let user_responses = users
                .into_iter()
                .map(|user| {
                    // Convert Option<chrono::DateTime<Utc>> to Option<String>
                    let registration_date = user.registration_date.unwrap();

                    UserModelResponse {
                        id: user.id,
                        username: user.username,
                        email: user.email,
                        registration_date: Some(registration_date),
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
                "message": err.to_string() // Use to_string() method directly
            });

            HttpResponse::InternalServerError().json(json_response)
        }
    }
}

#[derive(Debug, Deserialize ,Serialize)]
pub struct UserOperation {
    pub operation: String,
    pub result: bool,
}

#[post("/users/add/")]
pub async fn add_user(
    new_account: web::Json<UserAddRequestModel>,
    data: web::Data<AppState>,
) -> impl Responder {

    let username = &new_account.username;
    let password = &new_account.password;
    let email = &new_account.email;

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





pub fn filter_user_record(user: &UserModel) -> UserModelResponse {
    UserModelResponse {
        id: user.id,
        username: user.username.clone(),
        email: user.email.clone(),
        registration_date: user.registration_date, // Doğrudan Option<DateTime<Utc>>'ı kullanabilirsiniz
        // Diğer alanları da buraya ekleyebilirsin.
    }
}


#[derive(Debug, Deserialize)]
pub struct Nums {
    first: u64,
    second: u64,
}


#[derive(Debug, Serialize)]
struct Operation {
    operation: String,
    result: u64,
}

impl Responder for Operation {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

#[post("/add")]
pub async fn add(nums: web::Json<Nums>) -> impl Responder {
    Operation {
        operation: "add".to_string(),
        result: nums.first + nums.second,
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")

        .service(form_checker_handler)
        .service(full_form_message_list_handler)
        .service(create_message_handler)
        .service(one_line_get_users)
        .service(two_line_get_users)
        .service(add)
        .service(add_user)
        .service(form_message_list_by_form_name_handler);

    conf.service(scope);
}








