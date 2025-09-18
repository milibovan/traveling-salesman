use std::collections::HashSet;
use std::fs;
use crate::genetic_algorithm::evolution;
use crate::population::Population;

mod genetic_algorithm;
mod population;
mod tour;

pub const NO_CITIES: i32 = 4;
const MAX_GENERATIONS: i32 = 100;

#[derive(Debug, Eq, Hash, PartialEq)]
#[derive(Clone)]
struct Route {
    source: String,
    destination: String,
    distance: i32
}

fn main() {
    let (all_cities, routes) = read_cities_and_routes();
    let mut population = Population::new();

    let cities: Vec<String> = all_cities.iter().cloned().collect();

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

    if let Some(tour) = best_solution {
        println!("Best tour found after {} generations:", MAX_GENERATIONS);
        println!("Distance: {}", tour.total_distance);
    } else {
        println!("No solution found.");
    }
}

pub fn read_cities_and_routes() -> (HashSet<String>, HashSet<Route>) {
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