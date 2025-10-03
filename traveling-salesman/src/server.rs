use std::collections::HashSet;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use crate::genetic_algorithm::evolution;
use crate::{print_best_solution, GLOBALS};
use crate::marker::Marker;
use crate::population::Population;

#[post("/calculate-tour")]
async fn calculate_tour(markers: web::Json<Vec<Marker>>) -> impl Responder {
    println!("Received {} markers:", markers.len());

    for marker in markers.iter() {
        println!("Label: {}, Lat: {}, Lng: {}", marker.label, marker.coordinates.0, marker.coordinates.1);
    }

    ga_markers(markers.clone());

    web::Json(format!("Processed markers successfully!"))
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

pub fn ga_markers(markers: Vec<Marker>) {
    let mut population = Population::new();

    population.init_tours_markers(markers.clone());

    let mut best_solution = population
        .tours
        .iter()
        .min_by(|a, b| {
            a.total_distance.partial_cmp(&b.total_distance)
                .unwrap_or(std::cmp::Ordering::Less)
        })
        .cloned();

    for _ in 0..GLOBALS.max_generations {
        let new_population = evolution(&mut population, &HashSet::new(), markers.clone());

        if let Some(solution) = new_population
            .tours
            .iter()
            .min_by(|a, b| {
                a.total_distance.partial_cmp(&b.total_distance)
                    .unwrap_or(std::cmp::Ordering::Less)
            })
            .cloned()
        {
            if let Some(current_best) = &best_solution {
                if solution.total_distance < current_best.total_distance {
                    best_solution = Some(solution);
                }
            } else {
                best_solution = Some(solution);
            }
        }

        population = new_population;
    }

    print_best_solution(&mut best_solution);
}