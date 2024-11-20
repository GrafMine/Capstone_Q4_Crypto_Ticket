use actix_web::{post, HttpResponse, Responder};

#[post("lottery/create")]
async fn create() -> impl Responder {
    HttpResponse::Ok().json("Lottery created")
}
