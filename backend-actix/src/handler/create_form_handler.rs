// src/handlers.rs: 
use crate::AppState;


use actix_web::{post, web, HttpResponse, Responder};
use serde_json::json;

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


pub fn create_form_handler_config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(create_form_and_tables_handler);
    conf.service(scope);
}
