use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub last_login_date: Option<chrono::DateTime<chrono::Utc>>,
    pub registration_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserInfoModel{
    pub username: String,
    pub from_as_country: Option<String>,
    pub name: String,
    pub surname: String,
    pub sex: String,
    pub favorite_anime_girl: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserSecurityModelQuestion{
    pub username: String,
    pub user_using_question: bool,
    pub security_question: Option<String>,
    pub security_answer: Option<String>,
    pub updated_question_security_model_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserSecurityModelTelephoneNumber {
    pub username: String,
    pub user_using_number: bool,
    pub tel_number: Option<i32>,
    pub updated_telephone_number_security_model_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserSecurityModelSavingMail {
    pub username: String,
    pub user_using_saving_mail: bool,
    pub extra_mail: Option<String>,
    pub updated_saving_mail_security_model_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserModelSocials {
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub instagram: Option<String>,
    pub linkedin: Option<String>,
    pub personal_website: Option<String>,
    pub updated_user_model_socials_date: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModelResponse {
    pub user_id: i32, //Key
    pub username: String,
    pub email: String,
    pub last_login_date: Option<String>,
    pub registration_date: Option<String>,
}

#[derive(Debug, Deserialize ,Serialize)]
pub struct UserAddRequestModel{
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthUserRequestModelUsername {
    pub username: String,
    pub password: String, 
}
#[derive(Debug, Deserialize, Serialize)]
pub struct AuthUserRequestModelMail {
    pub email: String,
    pub password: String, 
}