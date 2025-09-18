use std::collections::HashSet;
use std::{env, fs, thread};
use std::sync::Arc;
use crate::genetic_algorithm::evolution;
use crate::population::Population;
use std::time::Instant;
use crate::tour::Tour;

mod genetic_algorithm;
mod population;
mod tour;

pub const NO_CITIES: i32 = 10;
const MAX_GENERATIONS: i32 = 100;
const MIGRATION_RATE: i32 = 5;

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
        // evolution must also accept the `routes` data
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

fn parallel_ga() {
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    let no_threads: i32 = args[1].parse().unwrap();

    let (all_cities, routes) = read_cities_and_routes();

    let cities: Vec<String> = all_cities
        .iter()
        .take(NO_CITIES as usize)
        .cloned()
        .collect();

    // let mut handles = vec![];

    for _ in 0..no_threads {
        let mut population = Population::new();

        population.init_tours(cities.clone(), &routes);}
    //
    //     let handle = thread::spawn(|| {
    //
    //         let mut best_solution = population
    //             .tours
    //             .iter()
    //             .min_by_key(|tour| tour.total_distance)
    //             .cloned();
    //
    //         for _ in 0..MAX_GENERATIONS {
    //             let new_population = evolution(&mut population, &routes);
    //
    //             if let Some(solution) = new_population
    //                 .tours
    //                 .iter()
    //                 .min_by_key(|tour| tour.total_distance)
    //                 .cloned()
    //             {
    //                 if let Some(current_best) = &best_solution {
    //                     if solution.total_distance < current_best.total_distance {
    //                         best_solution = Some(solution);
    //                     }
    //                 } else {
    //                     best_solution = Some(solution);
    //                 }
    //             }
    //
    //             population = new_population;
    //         }
    //
    //         print_best_solution(&mut best_solution);
    //         return best_solution.unwrap();
    //     });
    //
    //     handles.push(handle);
    // }
    //
    // for handle in handles {
    //     handle.join().unwrap();
    // }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}