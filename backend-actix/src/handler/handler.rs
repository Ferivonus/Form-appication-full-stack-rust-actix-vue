// src/handlers.rs: 
use crate::{
    schema::{CreateFormSchema, CreateMessageSchema, FilterAllMessagesOptions, FilterOnFormOptions},
    handler::models::{
        form_models::{
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
        user_models::{
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
    },
    AppState,
};


use serde::Deserialize;
use chrono::{ Utc, TimeZone};

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use serde_json::json;
use sqlx::{MySql, FromRow};

#[get("/live")]
async fn form_live_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Form system CRUD API with Rust, SQLX, MySQL, and Actix Web";

    HttpResponse::Ok().json(json!({"status": "success","form_message": MESSAGE}))
}

#[get("/anan")]
async fn anan_handler() -> impl Responder {
    const MESSAGE: &str = "Döne dolaşa buraya geldin, tebrikler, Ama ananın amı.";

    HttpResponse::Ok().json(json!({"status": "success","form_message": MESSAGE}))
}



#[post("/form/create/all_users/")]
async fn create_form_and_tables_handler(
    body: web::Json<CreateFormSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let form_title = body.form_title.to_owned().unwrap_or_default();
    let user_secret_string_id = body.user_secret_string_id.to_owned().unwrap_or_default();

    // Create chatting_form_messages_random_string table
    let create_random_string_table_query = format!(
        r#"CREATE TABLE IF NOT EXISTS {}_form_messages_random_string (
            id INT AUTO_INCREMENT PRIMARY KEY,
            random_string_to_get_id_after_create VARCHAR(255) NOT NULL UNIQUE
        )"#,
        &form_title
    );

    let _query_result = sqlx::query(&create_random_string_table_query)
        .execute(&data.db)
        .await
        .map_err(|err| err.to_string());

    // Create chatting_form_messages_answered_counter table
    let create_answered_counter_table_query = format!(
        r#"CREATE TABLE IF NOT EXISTS {}_form_messages_answered_counter (
            random_string_identifier VARCHAR(255) NOT NULL,
            answered_count INT NOT NULL DEFAULT 0,
            last_answered_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            FOREIGN KEY (random_string_identifier) REFERENCES {}_form_messages_random_string(random_string_to_get_id_after_create)
        )"#,
        &form_title, &form_title
    );

    let _query_result2 = sqlx::query(&create_answered_counter_table_query)
        .execute(&data.db)
        .await
        .map_err(|err| err.to_string());

    // Create chatting_form_messages_publishing_control table
    let create_publishing_control_table_query = format!(
        r#"CREATE TABLE IF NOT EXISTS {}_form_messages_publishing_control (
            random_string_identifier VARCHAR(255) NOT NULL,
            published BOOLEAN DEFAULT TRUE,
            publishing_detailes_changed_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            FOREIGN KEY (random_string_identifier) REFERENCES {}_form_messages_random_string(random_string_to_get_id_after_create)
        )"#,
        &form_title, &form_title
    );

    let _query_result3 = sqlx::query(&create_publishing_control_table_query)
        .execute(&data.db)
        .await
        .map_err(|err| err.to_string());

    // Create chatting_form_messages_message_info table
    let create_message_info_table_query = format!(
        r#"CREATE TABLE IF NOT EXISTS {}_form_messages_message_info (
            random_string_identifier VARCHAR(255) NOT NULL,
            sender_user_id INT NOT NULL,
            title VARCHAR(255) DEFAULT NULL,
            content TEXT NOT NULL,
            FOREIGN KEY (random_string_identifier) REFERENCES {}_form_messages_random_string(random_string_to_get_id_after_create)
        )"#,
        &form_title, &form_title
    );

    let _query_result4 = sqlx::query(&create_message_info_table_query)
        .execute(&data.db)
        .await
        .map_err(|err| err.to_string());

    // Create chatting_form_messages_has_image_information table
    let create_has_image_info_table_query = format!(
        r#"CREATE TABLE IF NOT EXISTS {}_form_messages_has_image_information (
            random_string_identifier VARCHAR(255) NOT NULL,
            has_image BOOLEAN DEFAULT FALSE,
            FOREIGN KEY (random_string_identifier) REFERENCES {}_form_messages_random_string(random_string_to_get_id_after_create)
        )"#,
        &form_title, &form_title
    );

    let _query_result5 = sqlx::query(&create_has_image_info_table_query)
        .execute(&data.db)
        .await
        .map_err(|err| err.to_string());

    let create_form_messages_image_counter = format!(
        r#"CREATE TABLE IF NOT EXISTS {}_form_messages_image_counter (
            counter_of_image INT AUTO_INCREMENT PRIMARY KEY,
	        random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
	        FOREIGN KEY (random_string_identifier) REFERENCES {}_form_messages_random_string(random_string_to_get_id_after_create)
        )"#,
        &form_title, &form_title
    );


    let _query_result6 = sqlx::query(&create_form_messages_image_counter)
        .execute(&data.db)
        .await
        .map_err(|err| err.to_string());

    // create chatting image informations
    let create_form_messages_image_counter = format!(
        r#"CREATE TABLE IF NOT EXISTS {}_form_messages_image_information (
            random_string_identifier VARCHAR(255) NOT NULL, -- to understand which message is it.
            image_data MEDIUMBLOB NOT NULL,
            image_name VARCHAR(255) NOT NULL,
            image_sender_username VARCHAR(255) NOT NULL,
	        FOREIGN KEY (random_string_identifier) REFERENCES {}_form_messages_random_string(random_string_to_get_id_after_create)
        )"#,
        &form_title, &form_title
    );

    let _query_result7 = sqlx::query(&create_form_messages_image_counter)
    .execute(&data.db)
    .await
    .map_err(|err| err.to_string());

    // Create chatting_form_messages_image_how_many_times_answered table
    let create_image_how_many_times_answered_table_query = format!(
        r#"CREATE TABLE IF NOT EXISTS {}_form_messages_image_how_many_times_answered (
            random_string_identifier VARCHAR(255) NOT NULL,
            answered_count INT NOT NULL DEFAULT 0,
            last_answer_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            FOREIGN KEY (random_string_identifier) REFERENCES {}_form_messages_random_string(random_string_to_get_id_after_create)
        )"#,
        &form_title, &form_title
    );

    let _query_result8 = sqlx::query(&create_image_how_many_times_answered_table_query)
    .execute(&data.db)
    .await
    .map_err(|err| err.to_string());

    // Create chatting_form_messages_image_like_dislake_founded_funny table
    let create_image_like_dislike_funny_table_query = format!(
        r#"CREATE TABLE IF NOT EXISTS {}_form_messages_image_like_dislake_founded_funny (
            random_string_identifier VARCHAR(255) NOT NULL,
            image_liked_count INT NOT NULL DEFAULT 0,
            image_disliked_count INT NOT NULL DEFAULT 0,
            image_founded_funny_count INT NOT NULL DEFAULT 0,
            FOREIGN KEY (random_string_identifier) REFERENCES {}_form_messages_random_string(random_string_to_get_id_after_create)
        )"#,
        &form_title, &form_title
    );

    let _query_result9 = sqlx::query(&create_image_like_dislike_funny_table_query)
    .execute(&data.db)
    .await
    .map_err(|err| err.to_string());

    // Create chatting_form_messages_like_dislake_information table
    let create_like_dislike_information_table_query = format!(
        r#"CREATE TABLE IF NOT EXISTS {}_form_messages_like_dislake_information (
            random_string_identifier VARCHAR(255) NOT NULL,
            liked_count INT NOT NULL DEFAULT 0,
            disliked_count INT NOT NULL DEFAULT 0,
            founded_funny INT NOT NULL DEFAULT 0,
            FOREIGN KEY (random_string_identifier) REFERENCES {}_form_messages_random_string(random_string_to_get_id_after_create)
        )"#,
        &form_title, &form_title
    );

    let _query_result10 = sqlx::query(&create_like_dislike_information_table_query)
    .execute(&data.db)
    .await
    .map_err(|err| err.to_string());

    // Create chatting_form_messages_message_time_info table
    let create_message_time_info_table_query = format!(
        r#"CREATE TABLE IF NOT EXISTS {}_form_messages_message_time_info (
            random_string_identifier VARCHAR(255) NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            FOREIGN KEY (random_string_identifier) REFERENCES {}_form_messages_random_string(random_string_to_get_id_after_create)
        )"#,
        &form_title, &form_title
    );

    let _query_result11 = sqlx::query(&create_message_time_info_table_query)
    .execute(&data.db)
    .await
    .map_err(|err| err.to_string());

    // Create chatting_form_messages_image_time_infos table
    let create_image_time_infos_table_query = format!(
        r#"CREATE TABLE IF NOT EXISTS {}_form_messages_image_time_infos (
            random_string_identifier VARCHAR(255) NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            changed_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            FOREIGN KEY (random_string_identifier) REFERENCES {}_form_messages_random_string(random_string_to_get_id_after_create)
        )"#,
        &form_title, &form_title
    );

    let _query_result12 = sqlx::query(&create_image_time_infos_table_query)
    .execute(&data.db)
    .await
    .map_err(|err| err.to_string());

    // Create chatting_form_messages_answered_to_node table
    let create_answered_to_node_table_query = format!(
        r#"CREATE TABLE IF NOT EXISTS {}_form_messages_answered_to_node (
            random_string_identifier VARCHAR(255) NOT NULL,
            answered_messages_string_value VARCHAR(255) NOT NULL,
            FOREIGN KEY (random_string_identifier) REFERENCES {}_form_messages_random_string(random_string_to_get_id_after_create)
        )"#,
        &form_title, &form_title
    );

    let _query_result13 = sqlx::query(&create_answered_to_node_table_query)
    .execute(&data.db)
    .await
    .map_err(|err| err.to_string());

    // Create chatting_form_answered_messages_info table
    let create_answered_messages_info_table_query = format!(
        r#"CREATE TABLE IF NOT EXISTS {}_form_answered_messages_info (
            random_string_identifier VARCHAR(255) NOT NULL,
            title_of_answered_message VARCHAR(255) NOT NULL,
            content_of_answered_message VARCHAR(255) NOT NULL,
            FOREIGN KEY (random_string_identifier) REFERENCES {}_form_messages_random_string(random_string_to_get_id_after_create)
        )"#,
        &form_title, &form_title
    );

    let _query_result14 = sqlx::query(&create_answered_messages_info_table_query)
    .execute(&data.db)
    .await
    .map_err(|err| err.to_string());

    // If all tables are created successfully, return success response
    HttpResponse::Ok().json(json!({
        "status": "success",
        "form_message": "Tables created successfully"
    }))
}


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
    let limit_count  = opts.limit.unwrap_or(10);
    let offset_count  = (opts.page.unwrap_or(1) - 1) * limit_count;

    // If form_title is present and not equal to form_name_of_link, return a bad request
    if let Some(ref form_title) = opts.form_title {
        if form_name_of_link != *form_title {
            return HttpResponse::BadRequest().json(json!({
                "status": "fail",
                "message": "form_name_of_link and form_title must be the same"
            }));
        }
    }


    let sql_query = format!(
        "SELECT * FROM {}_form_messages WHERE published = true ORDER by id LIMIT ? OFFSET ?",
        form_name_of_link
    );
    
    let messages: Vec<MessageInfoModel> = sqlx::query_as::<MySql, MessageInfoModel>(&sql_query)
        .bind(limit_count as i32)
        .bind(offset_count as i32)
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

        .service(anan_handler)
        .service(form_live_checker_handler)
        .service(every_message_handler)
        .service(add_message_handler)
        .service(get_user_by_username)
        .service(get_user_by_email)
        .service(add_user)
        .service(create_form_and_tables_handler)
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