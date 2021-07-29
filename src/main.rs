mod knapsack;
mod ga;

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let n_individuals: usize = 20;
    let n_generations: usize = 1000;
    let crossover_rate: f64 = 0.85;
    let mutation_rate: f64 = 0.03;
    let elitism_rate: f64 = 0.05;

    let n_datasets: usize = 15;
    let n_runs: usize = 30;

    for i in 1..n_datasets + 1 {
        let k: knapsack::Knapsack = knapsack::get_knapsack(i);
        let optimum: f64 = k.optimum;
        let n_genes: usize = k.weights.len();
        let g: ga::GA = ga::GA {
            k, n_individuals, n_genes, n_generations, 
            crossover_rate, mutation_rate, elitism_rate
        };
        let mut best: Vec<f64> = Vec::with_capacity(n_runs);
        for _ in 1..n_runs + 1 {
            let b: ga::Individual = g.run();
            best.push(b.fitness);
        }
        let result: [f64; 3] = mean_best_worst(&best);
        println!("Dataset: {} Optimum: {} Mean: {} Best: {} Worst: {}", 
        i, optimum, result[0], result[1], result[2]);
    }
    let elapsed = now.elapsed();
    println!("This took {:.2?}", elapsed);
}

fn mean_best_worst(x: &[f64]) -> [f64; 3] {
    let mut best: f64 = *x.first().unwrap();
    let mut worst: f64 = *x.last().unwrap();
    let mut total: f64 = 0.0;
    for i in x.iter() {
        total += i;
        if best < *i {
            best = *i;
        }
        if worst > *i {
            worst = *i;
        }
    }
    return [total / x.len() as f64, best, worst];
}

