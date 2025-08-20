use crate::tour::{get_random_cities, read_cities_and_routes, Tour};

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
    let (cities, routes) = read_cities_and_routes();
    let selected_cities: Vec<String> = get_random_cities(cities).iter().cloned().collect();
    let tour = Tour::init_tour(selected_cities, routes);
    println!("{:?}", tour);
}