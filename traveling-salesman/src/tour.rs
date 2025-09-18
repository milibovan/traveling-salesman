use std::collections::HashSet;
use std::fs;
use rand::{rng, Rng};
use crate::{Route, NO_CITIES};

#[derive(Debug, Eq, Hash, PartialEq)]
#[derive(Clone)]
pub struct Tour {
    pub(crate) cities: Vec<String>,
    pub(crate) total_distance: i32
}

impl Tour {
    pub fn init_tour(selected_cities: Vec<String>) -> Tour {
        let (_, routes) = read_cities_and_routes();

        // let mut selected_tour_routes: Vec<Route> = Vec::new();

        let mut visited_cities: Vec<String> = Vec::new();

        while visited_cities.len() != selected_cities.len() {
            let index = rng().random_range(0..selected_cities.len());
            if !visited_cities.contains(&selected_cities[index]) {
                visited_cities.push(selected_cities[index].clone());
            }
        }

        let mut total_distance:f32 = 0f32;

        for i in 0..visited_cities.len().saturating_sub(1) {
            let source = &visited_cities[i];
            let destination = &visited_cities[i+1];

            if let Some(route) = routes.iter().find(|r| (r.source == *source && r.destination == *destination) || (r.source == *destination && r.destination == *source)) {
                total_distance += route.distance as f32;
            } else {
                total_distance += f32::INFINITY;
            }
            // println!("{} -> {}", source, destination);
        }

        let source = &visited_cities[visited_cities.len()-1];
        let destination = &visited_cities[0];

        if let Some(route) = routes.iter().find(|r| (r.source == *source && r.destination == *destination) || (r.source == *destination && r.destination == *source)) {
            total_distance += route.distance as f32;
        } else {
            total_distance += f32::INFINITY;
        }

        Tour {
            cities: visited_cities,
            total_distance: total_distance as i32
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

pub fn calculate_total_distance(tour: &mut Tour) {
    let (_, routes) = read_cities_and_routes();

    let mut total_distance:f32 = 0f32;

    for i in 0..tour.cities.len() {
        let source = &tour.cities[i];
        let destination = &tour.cities[i+1];

        if let Some(route) = routes.iter().find(|r| (r.source == *source && r.destination == *destination) || (r.source == *destination && r.destination == *source)) {
            total_distance += route.distance as f32;
        } else {
            total_distance += f32::INFINITY;
        }
    }

    let source = &tour.cities[tour.cities.len()-1];
    let destination = &tour.cities[0];

    if let Some(route) = routes.iter().find(|r| (r.source == *source && r.destination == *destination) || (r.source == *destination && r.destination == *source)) {
        total_distance += route.distance as f32;
    } else {
        total_distance += f32::INFINITY;
    }

    tour.total_distance = total_distance as i32;
}