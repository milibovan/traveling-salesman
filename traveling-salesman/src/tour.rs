use std::collections::HashSet;
use rand::{rng};
use rand::seq::SliceRandom;
use crate::{Route};

#[derive(Debug, Eq, Hash, PartialEq)]
#[derive(Clone)]
pub struct Tour {
    pub(crate) cities: Vec<String>,
    pub(crate) total_distance: i32
}

impl Tour {
    pub fn init_tour(selected_cities: Vec<String>, routes: &HashSet<Route>) -> Tour {
        let mut visited_cities: Vec<String> = selected_cities;
        visited_cities.shuffle(&mut rng());

        let mut tour = Tour {
            cities: visited_cities,
            total_distance: 0, // Initialize to 0, it will be calculated next
        };

        tour.calculate_tour_distance(routes);
        tour
    }

    pub fn calculate_tour_distance(&mut self, routes: &HashSet<Route>) {
        let mut total_distance: f32 = 0f32;
        for i in 0..self.cities.len() {
            let source = &self.cities[i];
            let destination = &self.cities[(i + 1) % self.cities.len()];

            if let Some(route) = routes.iter().find(|r| (r.source == *source && r.destination == *destination) || (r.source == *destination && r.destination == *source)) {
                total_distance += route.distance as f32;
            } else {
                total_distance += f32::INFINITY;
            }
        }
        self.total_distance = total_distance as i32;
    }
}