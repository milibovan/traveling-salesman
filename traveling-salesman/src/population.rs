use std::collections::HashSet;
use crate::Route;
use crate::tour::{get_random_cities, read_cities_and_routes, Tour};

pub struct Population {
    tours: HashSet<Tour>,
    fitness: f32,
}

impl Population {
    pub fn new() -> Self {
        Population {
            tours: HashSet::new(),
            fitness: 0.0,
        }
    }

    pub fn init_cities_and_routes(&mut self) -> (Vec<String>, HashSet<Route>) {
        let (cities, routes) = read_cities_and_routes();
        let selected_cities: Vec<String> = get_random_cities(cities).iter().cloned().collect();
        (selected_cities, routes)
    }

    pub fn init_tours(&mut self) {
        let (selected_cities, routes) = self.init_cities_and_routes();
        for _ in 0..10 {
            self.tours.insert(Tour::init_tour(selected_cities.clone(), routes.clone()));
        }
        println!("{:?}", self.tours);
    }
}