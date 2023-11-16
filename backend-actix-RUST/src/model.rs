use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct FormMessageModel {
    pub id: String,
    pub title: String,
    pub content: String,
    pub form_title: Option<String>,
    pub published: i8,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct FormMessageModelResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub form_title: String,
    pub published: bool,
    pub createdAt: chrono::DateTime<chrono::Utc>,
    pub updatedAt: chrono::DateTime<chrono::Utc>,
}


#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub registration_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModelResponse {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub registration_date: Option<chrono::DateTime<chrono::Utc>>, 
}

#[derive(Debug, Deserialize ,Serialize)]
pub struct UserAddRequestModel{
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthUserRequestModel {
    pub username: String, 
    pub password: String, 
}
