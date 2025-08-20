use crate::population::Population;

mod genetic_algorithm;
mod population;
mod tour;

pub const NO_CITIES: i32 = 4;

#[derive(Debug, Eq, Hash, PartialEq)]
#[derive(Clone)]
struct Route {
    source: String,
    destination: String,
    distance: i32
}

fn main() {
    let mut population = Population::new();
    population.init_tours();
}