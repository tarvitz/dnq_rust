use actix_web::{get, App, HttpServer, Responder, HttpResponse};

#[get("/")]
async fn hello() -> impl Responder {
	HttpResponse::Ok().body("Hello world!")
}

#[get("/readyz")]
async fn readyz() -> impl Responder {
	HttpResponse::Ok().body("ok")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(||{
		App::new()
			.service(hello)
			.service(readyz)
	})
		.workers(8)
		.bind("0.0.0.0:8443")?.run().await
}