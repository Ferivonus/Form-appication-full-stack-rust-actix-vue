use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

// Last version which I did last night:
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct FormMessageModel {
    pub id: i32,
    pub random_string_to_get_id_after_create: String,
    pub user_id: i32,
    pub form_title: String,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub answered_count: i32,
    pub is_answer: bool,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_updater_username: Option<String>, 
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FormMessageModelResponse {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub form_title: String,
    pub published: bool,
    pub create_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub last_updater_username: Option<String>, 
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RandomStringModel {
    pub id: i32,
    pub random_string_to_get_id_after_create: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnsweredCounterModel {
    pub random_string_identifier: String,
    pub answered_count: i32,
    pub last_answered_time: DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PublishingControlModel {
    pub random_string_identifier: String,
    pub published: bool,
    pub publishing_detailes_changed_time: DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct MessageInfoModel {
    pub random_string_identifier: String,
    pub sender_user_id: i32,
    pub title: Option<String>,
    pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageInfoModelResponse {
    pub random_string_identifier: String,
    pub sender_user_id: i32,
    pub title: Option<String>,
    pub content: String,
}


#[derive(Debug, Deserialize, Serialize)]
pub struct HasImageInfoModel {
    pub random_string_identifier: String,
    pub has_image: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageCounterModel {
    pub counter_of_image: i32,
    pub random_string_identifier: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageInformationModel {
    pub random_string_identifier: String,
    pub image_data: Vec<u8>, // Assuming you store binary data for images
    pub image_name: String,
    pub image_sender_username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageTimeInfoModel {
    pub random_string_identifier: String,
    pub created_at: DateTime<chrono::Utc>,
    pub changed_time: DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageHowManyTimesAnsweredModel {
    pub random_string_identifier: String,
    pub answered_count: i32,
    pub last_answer_time: DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ImageLikeDislikeFunnyModel {
    pub random_string_identifier: String,
    pub image_liked_count: i32,
    pub image_disliked_count: i32,
    pub image_founded_funny_count: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LikeDislikeInformationModel {
    pub random_string_identifier: String,
    pub liked_count: i32,
    pub disliked_count: i32,
    pub founded_funny: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MessageTimeInfoModel {
    pub random_string_identifier: String,
    pub created_at: DateTime<chrono::Utc>,
    pub updated_at: DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnsweredToNodeModel {
    pub random_string_identifier: String,
    pub answered_messages_string_value: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnsweredMessagesInfoModel {
    pub random_string_identifier: String,
    pub title_of_answered_message: String,
    pub content_of_answered_message: String,
}