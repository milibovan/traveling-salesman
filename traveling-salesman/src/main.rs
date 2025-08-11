use std::collections::HashSet;
use std::fs;
use rand::{rng, Rng};

mod genetic_algorithm;

const NO_CITIES: i32 = 4;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Route {
    source: String,
    destination: String,
    distance: i32
}

fn main() {
    let (cities, routes) = read_cities_and_routes();

    let (starting_city, selected_cities) = get_starting_city_and_cities(cities);

    let selected_routes = routes.iter().filter(|route| selected_cities.contains(&route.source) && selected_cities.contains(&route.destination) ).collect::<Vec<&Route>>();

    println!("Starting city: {:?}, selected cities: {:?}", starting_city, selected_cities);
    println!("Selected routes: {:?}", selected_routes);
    // println!("{:?}", cities);
    // println!("{:?}", routes);
}

fn get_starting_city_and_cities(cities: HashSet<String>) -> (String, HashSet<String>) {
    let mut i = 0;
    let mut cities_vec: Vec<_> = cities.iter().collect();
    let mut selected_cities = HashSet::new();
    loop {
        i += 1;

        let index = rng().random_range(0..cities_vec.len());
        let starting_city = cities_vec[index].clone();

        selected_cities.insert(starting_city.clone());
        cities_vec.remove(index);
        if i >= NO_CITIES {
            break (starting_city, selected_cities);
        }
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
