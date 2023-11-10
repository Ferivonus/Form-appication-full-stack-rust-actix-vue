// src/links/mod.rs

use actix_web::{web, HttpResponse, HttpRequest};
use rand::Rng;
use serde::{Serialize, ser::{Serializer, SerializeStruct}};
// Import Serializer for custom serialization

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/test")
            .route(web::get().to(|| async { HttpResponse::Ok().body("test") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/app")
            .route(web::get().to(|| async { HttpResponse::Ok().body("app") }))
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}

struct Greeting {
    message: String,
    love_score: u8,
}

impl Serialize for Greeting {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Greeting", 2)?;
        state.serialize_field("message", &self.message)?;
        state.serialize_field("love_score", &self.love_score)?;
        state.end()
    }
}

pub async fn greet_user(req: HttpRequest) -> HttpResponse {
    let name = req.match_info().get("name").unwrap_or("User");
    
    let love_score = rand::thread_rng().gen_range(0..=36);
    
    let greeting = Greeting {
        message: format!("Hello, {}!", name),
        love_score,
    };

    HttpResponse::Ok().json(greeting)
}
