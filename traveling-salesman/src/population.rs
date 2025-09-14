use std::collections::HashSet;
use std::fmt::Debug;
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

    pub fn init_cities(&mut self) -> Vec<String> {
        let (cities, _) = read_cities_and_routes();
        let selected_cities: Vec<String> = get_random_cities(cities).iter().cloned().collect();
        selected_cities
    }

    pub fn init_tours(&mut self) {
        let (selected_cities) = self.init_cities();
        for _ in 0..10 {
            self.tours.insert(Tour::init_tour(selected_cities.clone()));
        }
        self.get_fitness();
    }

    pub fn get_fittest(&mut self) -> f32 {
        let mut fitesst = 0.0;
        if let Some(best_tour) = self.tours.iter().min_by_key(|tour| tour.total_distance) {
            fitesst = best_tour.total_distance as f32;
        } else {
            fitesst = 0.0;
        }
        fitesst
    }

    pub fn get_fitness(&mut self) {
        for tour in self.tours.iter() {
            self.fitness += tour.total_distance as f32;
        }
    }
}