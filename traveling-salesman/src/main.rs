use std::collections::HashSet;
use std::fs;

mod genetic_algorithm;

#[derive(Debug)]
struct Route {
    source: String,
    destination: String,
    distance: i32
}

fn main() {

    let input = fs::read_to_string("input.txt").expect("Something went wrong while reading the file");
    let split = input.split("\n").collect::<Vec<&str>>();
    let mut cities = HashSet::new();
    let mut routes = <Vec<Route>>::new();

    for relation in split {
        let row = relation.split(",").collect::<Vec<&str>>();
        cities.insert(row[0].to_string());
        routes.push(Route {
            source: row[0].parse().unwrap(),
            destination: row[2].parse().unwrap(),
            distance: row[4].parse().unwrap()
        });
    }

    println!("{:?}", cities);
    println!("{:?}", routes);
}
