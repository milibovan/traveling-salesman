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
-t, --threads <INT>	Number of islands (threads)	14

-c, --cities <INT>	Number of cities in the problem	10

-g, --generations <INT>	Maximum number of generations	100

-m, --migration <INT>	Interval (in generations) between migrations	10

-n, --no_migrants <INT>	Number of individuals migrating	2

-e, --expected-mutation-possibility <FLOAT>	Probability of mutation per individual	0.2

-p, --population <INT>	Population size per island	20

### Example
`cargo run --release -- -t 8 -c 30 -g 500 -m 20 -n 4 -e 0.15 -p 100`


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
| Population; Generations | 10 cities | 15 cities | 20 cities |
| :--- | :--- | :--- | :--- |
| 10; 100 | 17ms; 6770 / 33ms; 6590 | 16.5ms; 11150 / 40.5ms; 10153 | 27.5ms; 13073 / 58ms; 11535 |
| 20; 100 | 68.5ms; 4002 / 33ms; 3842 | 38s; 10218 / 94.5ms; 9268 | 59ms; 10350 / 139.5ms; 11650 |
| 30; 100 | 36ms; 6508 / 80.5ms; 6078 | 56ms; 7905 / 148.5ms; 7385 | 169.5ms; 12663 / 226.5ms; 11680 |
| 10; 200 | 61.44ms; 6320 / 120.88ms; 6100 | 108ms; 9650 / 161ms; 9170 | 139.5ms; 12453 / 214ms; 12250 |
| 20; 200 | 142.89ms; 4538 / 194.90ms; 4298 | 74.63ms; 8623 / 189.63ms; 7580 | 88.94ms; 12333 / 224.37ms; 11175 |
| 30; 200 | 120.54ms; 6300 / 287.54ms; 5740 | 121.95ms; 9420 / 277.34ms; 9103 | 161.17ms; 12658 / 398.29ms; 11218 |
| 10; 500 | 66.32ms; 4498 / 160.55ms; 4498 | 131.47ms; 9353 / 369.62ms; 8808 | 162.06ms; 12390 / 440.97ms; 11840 |
| 20; 500 | 124.70ms; 5908 / 409.78ms; 5800 | 208.75ms; 9403 / 752.29ms; 8418 | 271.97ms; 11803 / 979.17ms; 11200 |
| 30; 500 | 260.52ms; 6005 / 624.14ms; 5455 | 366.70ms; 10185 / 1.22s; 8990 | 419.51ms; 12168 / 1.67s; 11508 |



(results will vary depending on random seeds)

## 📂 Project Structure
src/

├── main.rs          # CLI argument parsing, I/O, and parallel GA orchestration

├── genetic_algorithm.rs # GA core functions: selection, crossover, and mutation

├── population.rs    # Handles population management

├── tour.rs          # Tour representation and distance calculation

├── server.rs        # Server implementation for frontend


## 🔮 Future Improvements

Integration with TSPLIB datasets for standardized testing.

Implementation of a hybrid GA with a local search heuristic.

Experiment logging and benchmarking for better performance analysis.

***
# 🌐 Traveling Salesman Frontend

This project includes a React-based frontend  application designed for visualizing and interacting with the TSP solver.

It utilizes TypeScript for robust development and the Leaflet map library for geographical display.

## 🗺️ Features

Interactive City Selection: Users can select checkpoints directly on the map interface.

Search Bar Integration: Allows adding cities to the problem set via a search bar.

Visualization: Displays the solved TSP tour on the map.

Real-time Communication: Connects to the Rust server on port 8080 for problem-solving and visualization.

##💻 Installation & Usage

Make sure you have Node.js and npm installed. The project is built with Vite.

Navigate to the frontend directory (if separate, or as part of the main clone).

Install dependencies:

`npm install`

Run the development server:

`npm run dev`

The frontend server will typically run on http://localhost:5173/, connecting to the Rust TSP server, which runs by default on port 8080.

## 📂 Frontend Structure

The core frontend implementation is detailed in the following files:

traveling-salesman/

├── traveling-salesman             # Rust implementation

├── traveling-salesman-frontend    # Root directory for the React/TypeScript frontend

│   ├── src/

│   │   ├── components/   # React components (e.g., map, city search)

│   │   ├── App.tsx       # Main application component

│   │   └── main.tsx      # Entry point

│   ├── index.html

│   └── package.json

└── ...

## 📜 License

This project is released under an Open Source Academic License.
You are free to use, modify, and share this code for educational and research purposes.
For commercial or production use, please contact the author.
