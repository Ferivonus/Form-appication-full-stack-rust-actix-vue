use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateMessageSchema {
    pub title: String,
    pub content: String,
                                            #[serde(default)]
    pub form_title: Option<String>,
                                            #[serde(default)]
    pub published: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateMessageSchema {
                                            #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
                                            #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
                                            #[serde(skip_serializing_if = "Option::is_none")]
    pub form_title: Option<String>,
                                            #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
}
