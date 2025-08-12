use std::collections::HashSet;
use std::fs;
use rand::{rng, Rng};
use crate::{Route, NO_CITIES};

#[derive(Debug)]
pub struct Tour {
    cities: Vec<String>,
    routes: Vec<Route>,
    total_distance: i32
}

impl Tour {
    pub fn create_tour() -> Tour {
        let (cities, routes) = read_cities_and_routes();
        let selected_cities: Vec<String> = get_random_cities(cities).iter().cloned().collect();

        let selected_routes: Vec<Route> = routes
            .into_iter()
            .filter(|route| selected_cities.contains(&route.source) && selected_cities.contains(&route.destination))
            .collect();

        let total_distance = selected_routes.iter().map(|route| route.distance).sum();

        Tour {
            cities: selected_cities,
            routes: selected_routes,
            total_distance,
        }
    }
}

pub fn get_random_cities(cities: HashSet<String>) -> HashSet<String> {
    let mut i = 0;
    let mut cities_vec: Vec<_> = cities.iter().collect();
    let mut selected_cities = HashSet::new();
    loop {
        i += 1;

        let index = rng().random_range(0..cities_vec.len());

        selected_cities.insert(cities_vec[index].clone());
        cities_vec.remove(index);
        if i >= NO_CITIES {
            break selected_cities;
        }
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
