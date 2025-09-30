use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(server_implementation)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn server_implementation() -> impl Responder {
    let message = format!("Successfull!");
    HttpResponse::Ok().body(message)
}