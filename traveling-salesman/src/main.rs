use std::collections::HashSet;
use std::{env, fs, thread};
use std::sync::Arc;
use crate::genetic_algorithm::evolution;
use crate::population::Population;
use std::time::Instant;
use crate::tour::Tour;
use std::sync::mpsc::{channel, Sender, Receiver};

mod genetic_algorithm;
mod population;
mod tour;

pub const NO_CITIES: i32 = 10;
const MAX_GENERATIONS: i32 = 100;
const MIGRATION_RATE: i32 = 10;
const MIGRANTS: i32 = 2;

#[derive(Debug, Eq, Hash, PartialEq)]
#[derive(Clone)]
struct Route {
    source: String,
    destination: String,
    distance: i32
}

fn main() {
    let now = Instant::now();
    let (all_cities, routes) = read_cities_and_routes();
    let mut population = Population::new();

    let cities: Vec<String> = all_cities
        .iter()
        .take(NO_CITIES as usize)
        .cloned()
        .collect();

    population.init_tours(cities.clone(), &routes);

    let mut best_solution = population
        .tours
        .iter()
        .min_by_key(|tour| tour.total_distance)
        .cloned();

    for _ in 0..MAX_GENERATIONS {
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

    parallel_ga(all_cities, routes);
}

fn print_best_solution(best_solution: &mut Option<Tour>) {
    if let Some(tour) = best_solution {
        println!("Best tour found after {} generations:", MAX_GENERATIONS);
        println!("Distance: {}", tour.total_distance);
        println!("{:?}", tour);
    } else {
        println!("No solution found.");
    }
}

fn read_cities_and_routes() -> (HashSet<String>, HashSet<Route>) {
    let input = fs::read_to_string("input.txt").expect("Something went wrong while reading the file");
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
    let args: Vec<String> = env::args().collect();
    let mut no_threads:i32 = 14;
    if args.len() > 1 {
        no_threads= args[1].parse().unwrap();
    }

    let mut senders = Vec::<Sender<Vec<Tour>>>::new();
    let mut receivers = Vec::<Receiver<Vec<Tour>>>::new();

    for _ in 0..no_threads {
        let (tx, rx) = channel();
        senders.push(tx);
        receivers.push(rx);
    }

    let cities: Vec<String> = all_cities
        .iter()
        .take(NO_CITIES as usize)
        .cloned()
        .collect();

    let arc_routes = Arc::new(routes);
    let arc_cities = Arc::new(cities);

    let mut handles = vec![];

    for id in 0..no_threads {
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

            for generation in 0..MAX_GENERATIONS {
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

                if generation % MIGRATION_RATE == 0 && generation > 0 {
                    let migrants: Vec<Tour> = population.tours.iter().cloned().take(MIGRANTS as usize).collect();

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