use std::collections::HashSet;
use std::{fs, thread};
use std::sync::Arc;
use crate::genetic_algorithm::evolution;
use crate::population::Population;
use std::time::Instant;
use crate::tour::Tour;
use std::sync::mpsc::{channel, Sender, Receiver};
use clap::Parser;
use lazy_static::lazy_static;

mod genetic_algorithm;
mod population;
mod tour;
mod server;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 14)]
    threads: i32,

    #[arg(short, long, default_value_t = 10)]
    cities: i32,

    #[arg(short, long, default_value_t = 100)]
    generations: i32,

    #[arg(short, long, default_value_t = 10)]
    migration_rate: i32,

    #[arg(short, long, default_value_t = 2)]
    no_migrants: i32,

    #[arg(short, long, default_value_t = 0.2)]
    expected_mutation_possibility: f32,

    #[arg(short, long, default_value_t = 20)]
    population: i32,
}

#[derive(Debug, Eq, Hash, PartialEq)]
#[derive(Clone)]
struct Route {
    source: String,
    destination: String,
    distance: i32
}

struct MyConstants {
    no_threads: i32,
    no_cities: i32,
    max_generations: i32,
    migration_rate: i32,
    migrants: i32,
    mutation_possibility: f32,
    population_size: i32
}

lazy_static! {
    static ref GLOBALS: MyConstants = {
        let args = Args::parse();
        MyConstants {
            no_threads: args.threads,
            no_cities: args.cities,
            max_generations: args.generations,
            migration_rate: args.migration_rate,
            migrants: args.no_migrants,
            mutation_possibility: args.expected_mutation_possibility,
            population_size: args.population
        }
    };
}

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::post().to(server_implementation)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

async fn server_implementation() -> impl Responder {
    let message = format!("Successfull!");
    HttpResponse::Ok().body(message)
}

// fn main() {
//     let (all_cities, routes) = sequential_ga();
//
//     parallel_ga(all_cities, routes);
// }

fn sequential_ga() -> (HashSet<String>, HashSet<Route>) {
    let now = Instant::now();

    let (all_cities, routes) = read_cities_and_routes();
    let mut population = Population::new();

    let cities: Vec<String> = all_cities
        .iter()
        .take(GLOBALS.no_cities as usize)
        .cloned()
        .collect();

    population.init_tours(cities.clone(), &routes);

    let mut best_solution = population
        .tours
        .iter()
        .min_by_key(|tour| tour.total_distance)
        .cloned();

    for _ in 0..GLOBALS.max_generations {
        let new_population = evolution(&mut population, &routes);

        if let Some(solution) = new_population
            .tours
            .iter()
            .min_by_key(|tour| tour.total_distance)
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

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    (all_cities, routes)
}

fn print_best_solution(best_solution: &mut Option<Tour>) {
    if let Some(tour) = best_solution {
        println!("Best tour found after {} generations:", GLOBALS.max_generations);
        println!("Distance: {}", tour.total_distance);
        println!("{:?}", tour);
    } else {
        println!("No solution found.");
    }
}

fn read_cities_and_routes() -> (HashSet<String>, HashSet<Route>) {
    let input = fs::read_to_string("utils/input.txt").expect("Something went wrong while reading the file");
    let split = input.split("\n").collect::<Vec<&str>>();
    let mut cities = HashSet::new();
    let mut routes = <HashSet<Route>>::new();

    for relation in split {
        let row = relation.split(",").collect::<Vec<&str>>();
        cities.insert(row[0].to_string());
        routes.insert(Route {
            source: row[0].parse().unwrap(),
            destination: row[2].parse().unwrap(),
            distance: row[4].parse().unwrap()
        });
    }
    (cities, routes)
}

fn parallel_ga(all_cities: HashSet<String>, routes: HashSet<Route>) {
    let now = Instant::now();

    let mut senders = Vec::<Sender<Vec<Tour>>>::new();
    let mut receivers = Vec::<Receiver<Vec<Tour>>>::new();

    for _ in 0..GLOBALS.no_threads {
        let (tx, rx) = channel();
        senders.push(tx);
        receivers.push(rx);
    }

    let cities: Vec<String> = all_cities
        .iter()
        .take(GLOBALS.no_cities as usize)
        .cloned()
        .collect();

    let arc_routes = Arc::new(routes);
    let arc_cities = Arc::new(cities);

    let mut handles = vec![];

    for id in 0..GLOBALS.no_threads {
        let routes = Arc::clone(&arc_routes);
        let cities = Arc::clone(&arc_cities);

        let all_senders = senders.clone();
        let receiver = receivers.remove(0);

        let handle = thread::spawn(move || {
            let mut population = Population::new();
            population.init_tours((*cities).clone(), &routes);

            let mut best_solution = population
                .tours
                .iter()
                .min_by_key(|tour| tour.total_distance)
                .cloned();

            for generation in 0..GLOBALS.max_generations {
                let new_population = evolution(&mut population, &routes);

                if let Some(solution) = new_population
                    .tours
                    .iter()
                    .min_by_key(|tour| tour.total_distance)
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

                if generation % GLOBALS.migration_rate == 0 && generation > 0 {
                    let migrants: Vec<Tour> = population.tours.iter().cloned().take(GLOBALS.migrants as usize).collect();

                    for (i, sender) in all_senders.iter().enumerate() {
                        if i != id as usize{
                            let _ = sender.send(migrants.clone());
                        }
                    }
                }

                while let Ok(incoming) = receiver.try_recv() {
                    for migrant in incoming {
                        population.tours.push(migrant);
                    }
                }
            }


            print_best_solution(&mut best_solution);
            best_solution.unwrap()
        });

        handles.push(handle);
    }

    let results: Vec<Tour> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    println!("{:?}", results.iter().min_by_key(|tour| {tour.total_distance}));

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}