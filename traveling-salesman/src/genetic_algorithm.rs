// Genetic representation of solution
    // Generate Genome that is represented by 0s and 1s, one solution
    // Genome = List[int]

    // fn generate_genome() -> Genome {
    //     return random(0 or 1)
    // }

// Function to generate new solutions
    // Generate population which is just a list of Genomes
    // Population = List[Genome]

    // fn generate_population() -> Population {
    //      let population = []
    //      for i..3 {
    //          population.add(generate_genome())
    //      }
    //      return population
    // }

// Fitness function to evaluate solutions
    // Thing = ['source', 'dest', 'weight']
    //     fn fitness_function(genome: Genome, nodes: List[Nodes], visited_nodes: List[Nodes]) -> i32 {
    //         if len(genome) != len(nodes) {
    //             throw Error();
    //         }
    //         let value = 0;
    //
    //         for i, node in visited_nodes:
    //             if genome == 1:
    //                 value += genome.value
    //
    //         return value;
    //     }

// Selection function to select solutions for next generation
    //     Fitness_function = Callable([Genome], int)
    //     fn select_pair(population: Population, fitness_function: Fitness_function) -> Population {
    //         return choices ( population=population,
    //         we want those with higher weight to be more likely to be selected for next generation
    //         weights=[fitness_function(genome) for genome in population],
    //         k=2)
    //     }

// Crossover function to cut genomes and mix them
    //     fn single_point_crossover(a: Genome, b: Genome) -> Tuple[Genome, Genome] {
    //         if len(a) != len(b)
    //              throw Error
    //         if (len(a) < 2 || len(b) < 2) {
    //              return a, b
    //         }
    //         p = randint(1, len-1)
    //         return a[0:p] + b[p:], b[0:p] + a[p:];
    //     }

// Mutation with certain possibility mutate bits in solution
    // fn mutation(genome: Genome, num: i32 = 1, possibility = 0.5) -> Genome {
    //      for _ in range(num) {
    //          index = randrange(len(genome))
    //          if random() > possibility
    //              genome[index]
    //          else
    //              abs(genome[index] - 1)
    //      }
    // }

// Run evolution function
    // populate population
    // loop in generation_limit
    // if fitness function exceeds fitness limit break
    // get next generation best 2 out of current population
    // now iterate over half of current generation
        // we get parents with selection function (population, fitness_fn)
        // we get 2 genomes with crossover_function (parent[0], parent[1])
        // mutate first
        // mutate second
        // add to next_generation
    // population = next_generation

    // return population (tour), and number of populations