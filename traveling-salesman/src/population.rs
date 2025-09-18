use std::collections::HashSet;
use crate::genetic_algorithm::POPULATION_SIZE;
use crate::Route;
use crate::tour::{Tour};

pub struct Population {
    pub(crate) tours: Vec<Tour>,
    pub(crate) fitness: f32,
}

impl Population {
    pub fn new() -> Self {
        Population {
            tours: Vec::new(),
            fitness: 0.0,
        }
    }

    pub fn init_tours(&mut self, all_cities: Vec<String>, routes: &HashSet<Route>) {
        for _ in 0..POPULATION_SIZE {
            self.tours.push(Tour::init_tour(all_cities.clone(), routes));
        }
        for tour in self.tours.iter() {
            self.fitness += tour.total_distance as f32;
        }
    }
}