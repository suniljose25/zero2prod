use actix_web::web::Form;
use actix_web::{HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(_form: Form<FormData>) -> impl Responder {
    HttpResponse::Ok()
}
