use std::collections::HashSet;
use rand::{rng, Rng};
use crate::{Route, GLOBALS};
use crate::marker::Marker;
use crate::population::Population;
use crate::tour::{Tour};

// Selection function to select solutions for next generation
// TODO better selection for case of local optimum
fn selection(population: &Population) -> Vec<Tour> {
    let mut best_tours: Vec<Tour> = Vec::new();

    if let Some(best_tour) = population.tours.iter()
        .min_by(|a, b| {
            a.total_distance.partial_cmp(&b.total_distance)
                .unwrap_or(std::cmp::Ordering::Less)
        }) {
        best_tours.push(best_tour.clone());
    }

    if let Some(second_best_tour) =population.tours.iter().filter(|tour: &&Tour| {!best_tours.contains(&tour)}).min_by(|a, b| {
        a.total_distance.partial_cmp(&b.total_distance)
            .unwrap_or(std::cmp::Ordering::Less)
    }) {
        best_tours.push(second_best_tour.clone());
    };

    best_tours
}

// Crossover function to cut genomes and mix them
fn crossover(parent1: &Tour, parent2: &Tour, routes: &HashSet<Route>, mut no_cities: i32) -> Tour {
    if no_cities == 0 {
        no_cities = GLOBALS.no_cities;
    }

    let mut cities: Vec<String> = Vec::new();

    let pivot: usize = rng().random_range(0..no_cities) as usize;

    for i in 0..pivot {
        cities.push(parent1.cities[i].clone());
    }

    for city in parent2.cities.iter() {
        if !cities.contains(city) {
            cities.push(city.clone());
        }
    }

    let child = Tour::init_tour(cities, routes);
    child
}

// Mutation with certain possibility mutate bits in solution
fn mutation(tour: &mut Tour, no_cities: i32){
    let mut collection_size = 0;
    if no_cities != 0 {
        collection_size = no_cities;
    } else {
        collection_size = GLOBALS.no_cities;
    }
    for index1 in 0..collection_size {
        if rng().random_range(0.0..1.0) <= GLOBALS.mutation_possibility {
            let index2 = rng().random_range(0..collection_size) as usize;

            tour.cities.swap(index1 as usize, index2);
        }
    }
}

// Evolution, main loop of ga
pub fn evolution(population: &mut Population, routes: &HashSet<Route>, cities: Vec<Marker>) -> Population {
    let mut new_population = Population::new();

    let best_tours = selection(population);
    new_population.tours.push(best_tours[0].clone());
    new_population.tours.push(best_tours[1].clone());

    loop {
        if new_population.tours.len() == GLOBALS.population_size as usize {
            break;
        }
        let mut child = crossover(&best_tours[0], &best_tours[1], &routes, cities.len() as i32);
        mutation(&mut child, cities.len() as i32);
        new_population.tours.push(child);
    }

    if routes.len() == 0 {
        for tour in new_population.tours.iter_mut() {
            tour.calculate_tour_distance_with_markers(cities.clone());
        }
    } else {
        for tour in new_population.tours.iter_mut() {
            tour.calculate_tour_distance(routes);
        }
    }


    new_population
}