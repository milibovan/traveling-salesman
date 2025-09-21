# Traveling Salesman Problem in Rust

Genetic Algorithm with Parallelized Island Model

## 📌 Overview

This project implements a solver for the Traveling Salesman Problem (TSP) in Rust, using a Genetic Algorithm (GA).
To improve performance and solution quality, the solver is parallelized with the Island Model: multiple subpopulations (islands) evolve independently and occasionally exchange individuals (migration).

This combines:

Rust’s high performance

Heuristic optimization (GA)

Parallelism with island model

## 🚀 Features

TSP solved with a Genetic Algorithm

Island model parallelization with configurable number of islands (threads)

Adjustable GA parameters via CLI

Elitism to keep the best tours

Migration between islands after configurable generations

Designed for research, experimentation, and academic use

## 🛠️ Installation

Make sure you have Rust
 installed.

`git clone https://github.com/milibovan/traveling-salesman.git`
`cd traveling-salesman`
`cargo build --release`

### ▶️ Usage

Run the program with:

`cargo run --release -- [OPTIONS]`

### Arguments
Option	Description	Default
-t, --no-threads <INT>	Number of islands (threads)	14
-c, --no-cities <INT>	Number of cities in the problem	10
-g, --max-generations <INT>	Maximum number of generations	100
-r, --migration-rate <INT>	Interval (in generations) between migrations	10
-m, --migrants <INT>	Number of individuals migrating	2
-u, --mutation-possibility <FLOAT>	Probability of mutation per individual	0.2
-p, --population-size <INT>	Population size per island	20
### Example
`cargo run --release -- -t 8 -c 30 -g 500 -r 20 -m 4 -u 0.15 -p 100`


This runs with:

8 islands (threads)

30 cities

500 generations

Migration every 20 generations, exchanging 4 individuals

Mutation probability 15%

100 individuals per island

## ⚙️ Algorithm Details
Genetic Algorithm
Initialization → Random tours are generated.

**Selection** → The two fittest individuals from the current population are selected to become parents for the next generation.

**Crossover** → A single-point crossover is applied. A random pivot point is chosen, and the child's tour starts with a segment from the first parent, filling the remaining cities with those from the second parent in the order they appear.

**Mutation** → A swap mutation is used. For each city in the tour, there's a small chance of it being swapped with another randomly chosen city.

**Replacement** → Elitism is used, where the two best tours from the previous generation are carried over to the new population.

### Island Model
Each island evolves its population independently.

After every migration_rate generations, a fixed number of the best individuals (migrants) are cloned and sent to all other islands to improve their genetic diversity. This helps prevent premature convergence and find better solutions.

## 📊 Example Results


(results will vary depending on random seeds)

## 📂 Project Structure
src/

├── main.rs          # CLI argument parsing, I/O, and parallel GA orchestration

├── genetic_algorithm.rs # GA core functions: selection, crossover, and mutation

├── population.rs    # Handles population management

├── tour.rs          # Tour representation and distance calculation

## 🔮 Future Improvements

Integration with TSPLIB datasets for standardized testing.

Visualization of evolving tours.

Implementation of a hybrid GA with a local search heuristic.

Experiment logging and benchmarking for better performance analysis.

## 📜 License

This project is released under an Open Source Academic License.
You are free to use, modify, and share this code for educational and research purposes.
For commercial or production use, please contact the author.
