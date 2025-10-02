use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use crate::marker::Marker;

#[post("/calculate-tour")]
async fn calculate_tour(markers: web::Json<Vec<Marker>>) -> impl Responder {
    println!("Received {} markers:", markers.len());

    for marker in markers.iter() {
        println!("Label: {}, Lat: {}, Lng: {}", marker.label, marker.coordinates.0, marker.coordinates.1);
    }

    web::Json(format!("Processed {} markers successfully!", markers.len()))
}

async fn server_implementation() -> impl Responder {
    let message = "Status OK!".to_string();
    HttpResponse::Ok().body(message)
}

pub async fn start_server() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![actix_web::http::header::AUTHORIZATION, actix_web::http::header::ACCEPT, actix_web::http::header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(calculate_tour)
            .route("/api-status", web::get().to(server_implementation))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await.expect("Server not started");

    Ok(())
}