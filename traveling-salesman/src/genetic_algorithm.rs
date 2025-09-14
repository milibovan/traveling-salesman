use rand::{rng, Rng};
use crate::NO_CITIES;
use crate::tour::{read_cities_and_routes, Tour};
// Selection function to select solutions for next generation
//     Fitness_function = Callable([Genome], int)
//     fn select_pair(population: Population, fitness_function: Fitness_function) -> Population {
//         return choices ( population=population,
//         we want those with higher weight to be more likely to be selected for next generation
//         weights=[fitness_function(genome) for genome in population],
//         k=2)
//     }


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
// fn mutation(genome: Genome, num: i32 = 1, possibility = 0.5) -> Genome {
//      for _ in range(num) {
//          index = randrange(len(genome))
//          if random() > possibility
//              genome[index]
//          else
//              abs(genome[index] - 1)
//      }
// }

// Run evolution function
// populate population
// loop in generation_limit
// if fitness function exceeds fitness limit break
// get next generation best 2 out of current population
// now iterate over half of current generation
// we get parents with selection function (population, fitness_fn)
// we get 2 genomes with crossover_function (parent[0], parent[1])
// mutate first
// mutate second
// add to next_generation
// population = next_generation

// return population (tour), and number of populations