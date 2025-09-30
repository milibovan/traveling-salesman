use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn server_implementation() -> impl Responder {
    // You could integrate your GA logic here or access its results
    let message = format!("Successfull!");
    HttpResponse::Ok().body(message)
}

pub async fn start_server() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(|| {
        App::new().route("/", web::get().to(server_implementation))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}