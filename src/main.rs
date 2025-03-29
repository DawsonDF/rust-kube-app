use actix_web::{App, HttpResponse, HttpServer, Responder, get};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"status":"success","message":"Testing!"}"#)
}

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("healthy")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://0.0.0.0:8080");

    HttpServer::new(|| App::new().service(index).service(health_check))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
