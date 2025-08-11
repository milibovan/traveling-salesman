use std::collections::HashSet;
use std::fs;
use rand::{rng, Rng};

mod genetic_algorithm;

#[derive(Debug, Eq, Hash, PartialEq)]
struct Route {
    source: String,
    destination: String,
    distance: i32
}

fn main() {
    let (cities, routes) = read_cities_and_routes();

    let starting_city = get_starting_city(cities);
    println!("Starting city: {:?}", starting_city);
    // println!("{:?}", cities);
    // println!("{:?}", routes);
}

fn get_starting_city(cities: HashSet<String>) -> String {
    let cities_vec: Vec<_> = cities.iter().collect();
    let index = rng().random_range(0..cities_vec.len());

    cities_vec[index].clone()
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
