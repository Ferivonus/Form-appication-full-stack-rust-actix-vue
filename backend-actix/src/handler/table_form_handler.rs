// src/handlers.rs: 
use crate::AppState;


use actix_web::{delete, get, put ,patch, post, web, HttpResponse, Responder};
use serde_json::json;
use sqlx::MySql;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct  CreateFormSchema{
    pub form_title: Option<String>,
    pub user_secret_string_id: Option<String>,
}

#[post("/form/create/all_users/")]
async fn create_form_and_tables_handler(
    body: web::Json<CreateFormSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let form_title = body.form_title.to_owned().unwrap_or_default();
    // let user_secret_string_id = body.user_secret_string_id.to_owned().unwrap_or_default();

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


#[derive(serde::Deserialize, Debug, sqlx::FromRow)]
pub struct  DropFormTablesRequest{
    pub form_title: String,
    pub admin_name: String,
    pub admin_password: String,
    pub admin_secret_key: String,
    pub getter_token_from_main_admin: String
}

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct AdminUserModel {
    pub id: i32,
    pub admin_name: String,
    pub admin_password: String,
    pub admin_secret_key: String,
    pub getter_token_from_main_admin: String,
    pub used_time: i32,
    pub last_answer_time: Option<chrono::NaiveDateTime>,
}


// Define the drop tables endpoint
#[delete("/form/drop/admin/")]
async fn drop_form_tables_handler(
    body: web::Json<DropFormTablesRequest>,
    data: web::Data<AppState>,
) -> impl Responder {
    // Extract values from the request body
    let form_title = &body.form_title;
    let admin_name = &body.admin_name;
    let admin_password = &body.admin_password;
    let admin_secret_key = &body.admin_secret_key;
    let getter_token_from_main_admin = &body.getter_token_from_main_admin;

    // Prepare the SQL query with placeholders to prevent SQL injection
    let sql_query = "\
        SELECT * FROM admin_authentication_table_for_drop_form \
        WHERE admin_name = ? AND admin_password = ? AND admin_secret_key = ? AND gotten_token_from_main_admin = ?";

    // Execute the query
    let user: Result<Option<AdminUserModel>, _> = sqlx::query_as::<MySql, AdminUserModel>(&sql_query)
        .bind(admin_name)
        .bind(admin_password)
        .bind(admin_secret_key)
        .bind(getter_token_from_main_admin)
        .fetch_optional(&data.db)
        .await;


    // Check if a user with the provided credentials exists
    match user {
        Ok(Some(_)) => {
            // User found, proceed with dropping tables
            // Call the function to drop tables
            drop_tables(&form_title, &data.db).await;

            let sql_update_query = "\
                UPDATE admin_authentication_table_for_drop_form \
                SET used_time = used_time + 1 \
                WHERE admin_name = ?";

            // Execute the update query
            let _ = sqlx::query(&sql_update_query)
                .bind(admin_name)
                .execute(&data.db)
                .await;

            // Return success response
            HttpResponse::Ok().json(json!({
                "status": "success",
                "message": "Tables dropped successfully"
            }))
        }
        _ => {
            // Invalid credentials or user not found
            HttpResponse::Unauthorized().json(json!({
                "status": "error",
                "message": "Invalid credentials or user not found"
            }))
        }
    }

}

async fn drop_tables(form_title: &str, db: &sqlx::MySqlPool) {

    // Drop all the tables associated with the given form_title
    let drop_publishing_control_table_query = format!(
        "DROP TABLE IF EXISTS {}_form_messages_publishing_control",
        &form_title
    );
    let _ = sqlx::query(&drop_publishing_control_table_query)
        .execute(db)
        .await
        .map_err(|err| eprintln!("Error dropping publishing control table: {}", err));

    let drop_message_info_table_query = format!(
        "DROP TABLE IF EXISTS {}_form_messages_message_info",
        &form_title
    );
    let _ = sqlx::query(&drop_message_info_table_query)
        .execute(db)
        .await
        .map_err(|err| eprintln!("Error dropping message info table: {}", err));

    let drop_has_image_info_table_query = format!(
        "DROP TABLE IF EXISTS {}_form_messages_has_image_information",
        &form_title
    );
    let _ = sqlx::query(&drop_has_image_info_table_query)
        .execute(db)
        .await
        .map_err(|err| eprintln!("Error dropping has image info table: {}", err));

    let drop_image_counter_table_query = format!(
        "DROP TABLE IF EXISTS {}_form_messages_image_counter",
        &form_title
    );
    let _ = sqlx::query(&drop_image_counter_table_query)
        .execute(db)
        .await
        .map_err(|err| eprintln!("Error dropping image counter table: {}", err));

    let drop_image_info_table_query = format!(
        "DROP TABLE IF EXISTS {}_form_messages_image_information",
        &form_title
    );
    let _ = sqlx::query(&drop_image_info_table_query)
        .execute(db)
        .await
        .map_err(|err| eprintln!("Error dropping image info table: {}", err));

    let drop_image_time_infos_table_query = format!(
        "DROP TABLE IF EXISTS {}_form_messages_image_time_infos",
        &form_title
    );
    let _ = sqlx::query(&drop_image_time_infos_table_query)
        .execute(db)
        .await
        .map_err(|err| eprintln!("Error dropping image time infos table: {}", err));

    let drop_image_how_many_times_answered_table_query = format!(
        "DROP TABLE IF EXISTS {}_form_messages_image_how_many_times_answered",
        &form_title
    );
    let _ = sqlx::query(&drop_image_how_many_times_answered_table_query)
        .execute(db)
        .await
        .map_err(|err| eprintln!("Error dropping image how many times answered table: {}", err));

    let drop_image_like_dislike_funny_table_query = format!(
        "DROP TABLE IF EXISTS {}_form_messages_image_like_dislake_founded_funny",
        &form_title
    );
    let _ = sqlx::query(&drop_image_like_dislike_funny_table_query)
        .execute(db)
        .await
        .map_err(|err| eprintln!("Error dropping image like dislike funny table: {}", err));

    let drop_like_dislike_information_table_query = format!(
        "DROP TABLE IF EXISTS {}_form_messages_like_dislake_information",
        &form_title
    );
    let _ = sqlx::query(&drop_like_dislike_information_table_query)
        .execute(db)
        .await
        .map_err(|err| eprintln!("Error dropping like dislike information table: {}", err));

    let drop_message_time_info_table_query = format!(
        "DROP TABLE IF EXISTS {}_form_messages_message_time_info",
        &form_title
    );
    let _ = sqlx::query(&drop_message_time_info_table_query)
        .execute(db)
        .await
        .map_err(|err| eprintln!("Error dropping message time info table: {}", err));

    let drop_answered_to_node_table_query = format!(
        "DROP TABLE IF EXISTS {}_form_messages_answered_to_node",
        &form_title
    );
    let _ = sqlx::query(&drop_answered_to_node_table_query)
        .execute(db)
        .await
        .map_err(|err| eprintln!("Error dropping answered to node table: {}", err));

    let drop_answered_messages_info_table_query = format!(
        "DROP TABLE IF EXISTS {}_form_answered_messages_info",
        &form_title
    );
    let _ = sqlx::query(&drop_answered_messages_info_table_query)
        .execute(db)
        .await
        .map_err(|err| eprintln!("Error dropping answered messages info table: {}", err));
    
}

pub fn table_form_handler_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(create_form_and_tables_handler);
    conf.service(scope);
}
