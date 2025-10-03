use std::collections::HashSet;
use rand::{rng};
use rand::seq::SliceRandom;
use serde::Serialize;
use crate::{Route};
use crate::marker::Marker;

#[derive(Debug, PartialEq, Serialize)]
#[derive(Clone)]
pub struct Tour {
    pub(crate) cities: Vec<String>,
    pub(crate) total_distance: f32
}

impl Tour {
    pub fn init_tour(selected_cities: Vec<String>, routes: &HashSet<Route>) -> Tour {
        let mut visited_cities: Vec<String> = selected_cities;
        visited_cities.shuffle(&mut rng());

        let mut tour = Tour {
            cities: visited_cities,
            total_distance: 0.0,
        };

        tour.calculate_tour_distance(routes);
        tour
    }

    pub fn init_tour_with_markers(mut cities: Vec<Marker>) -> Tour {
        cities.shuffle(&mut rng());

        let tour_cities: Vec<String> = cities.iter()
            .map(|marker| marker.label.clone())
            .collect();

        let mut tour = Tour {
            cities: tour_cities,
            total_distance: 0.0,
        };

        tour.calculate_tour_distance_with_markers(cities);

        tour
    }

    pub fn calculate_tour_distance_with_markers(&mut self, cities: Vec<Marker>) {
        let num_cities = self.cities.len();

        if num_cities < 2 {
            return;
        }

        for i in 0..num_cities - 1 {
            self.total_distance += Tour::haversine_distance(
                cities[i].coordinates,
                cities[i + 1].coordinates
            ) ;
        }

        self.total_distance += Tour::haversine_distance(
            cities[num_cities - 1].coordinates,
            cities[0].coordinates
        );
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
        self.total_distance = total_distance;
    }

    pub fn haversine_distance(city1: (f32, f32), city2: (f32, f32)) -> f32{
        const R: f32 = 6371.0;

        let lat1 = city1.0 ;
        let lat2 = city2.0 ;
        let lon1 = city1.1 ;
        let lon2 = city2.1 ;


        let lat1_rad = lat1 * (std::f32::consts::PI / 180.0);
        let lat2_rad = lat2  * (std::f32::consts::PI / 180.0);
        let lon1_rad = lon1  * (std::f32::consts::PI / 180.0);
        let lon2_rad = lon2  * (std::f32::consts::PI / 180.0);

        let dlat = lat2_rad - lat1_rad;
        let dlon = lon2_rad - lon1_rad;

        let a = f32::sin(dlat / 2.0).powi(2) + f32::cos(lat1_rad) * f32::sin(dlon / 2.0).powi(2);
        let c = 2.0 * f32::atan2(a.sqrt(), (1.0-a).sqrt());

        let distance: f32 = R * c;
        distance
    }

}