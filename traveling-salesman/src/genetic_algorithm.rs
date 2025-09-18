use rand::{rng, Rng};
use crate::{NO_CITIES};
use crate::population::Population;
use crate::tour::{calculate_total_distance, Tour};
const MUTATION_POSSIBILITY: f32 = 0.2;

const POPULATION_SIZE: i32 = 20;

// Selection function to select solutions for next generation
fn selection(population: &mut Population) -> Vec<Tour> {
    let mut best_tours: Vec<Tour> = Vec::new();

    if let Some(best_tour) = population.tours.iter().min_by_key(|tour| tour.total_distance) {
        best_tours.push(best_tour.clone());
    }

    if let Some(second_best_tour) =population.tours.iter().filter(|tour: &&Tour| {!best_tours.contains(&tour)}).min_by_key(|tour| tour.total_distance) {
        best_tours.push(second_best_tour.clone());
    };

    best_tours
}

// Crossover function to cut genomes and mix them
fn crossover(parent1: &Tour, parent2: &Tour) -> Tour {
    let mut cities: Vec<String> = Vec::new();

    let pivot:usize = rng().random_range(0..NO_CITIES) as usize;

    let mut i:usize = 0;
    loop {
        i += 1;
        if i == pivot {
            break;
        }
        cities.push(parent1.cities[i].clone());
    }

    for city in parent2.cities.iter() {
        if !cities.contains(&city) {
            cities.push(city.clone());
        }
    }

    let child = Tour::init_tour(cities);
    child
}

// Mutation with certain possibility mutate bits in solution
fn mutation(tour: &mut Tour){
    for index1 in 0..NO_CITIES {
        if rng().random_range(0.0..1.0) <= MUTATION_POSSIBILITY {
            let index2 = rng().random_range(0..NO_CITIES) as usize;

            let city1 = tour.cities[index1 as usize].clone();
            let city2 = tour.cities[index2].clone();

            if !city1.eq(&city2) {
                tour.cities[index1 as usize] = city2;
                tour.cities[index2] = city1;
            }
        }
    }
}

// Evolution, main loop of ga
fn evolution(population: &mut Population) -> Population {
    let mut new_population = Population::new();

    // selection
    let best_tours = selection(population);

    new_population.tours.push(best_tours[0].clone());
    new_population.tours.push(best_tours[1].clone());

    loop {
        if new_population.tours.len() == POPULATION_SIZE as usize {
            break;
        }
        let mut child = crossover(&best_tours[0], &best_tours[1]);

        mutation(&mut child);

        new_population.tours.push(child);
    }

    for tour in new_population.tours.iter_mut() {
        calculate_total_distance(tour);
    }

    new_population
}