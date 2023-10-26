// use log::{info};
use actix_web::{get, post, Responder, HttpResponse, HttpRequest};
use log::{info};

#[get("/")]
pub async fn hello() -> impl Responder {
	info!("default endpoint");

	HttpResponse::Ok().body("Hello world!")
}

#[get("/readyz")]
pub async fn readyz() -> impl Responder {
	HttpResponse::Ok().body("ok")
}

#[post("/answer")]
pub async fn answer() -> impl Responder {
	HttpResponse::Ok().body("ok")
}