/*
 * ga.rs
 *
 * Copyright (C) 2021 Sharaz Ali
 *
 * This program is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the Free
 * Software Foundation, either version 3 of the License, or (at your option)
 * any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of  MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
 * more details.
 *
 * You should have received a copy of the GNU General Public License along with
 * this program.  If not, see <http://www.gnu.org/licenses/>.
 */

extern crate rand;
use rand::Rng;
use crate::knapsack;

pub struct Individual {
    pub genes: Vec<bool>,
    pub fitness: f64
}

impl Individual {
    fn clone(&self) -> Individual {
        return Individual{genes: self.genes.clone(), fitness: self.fitness};
    }
}

pub struct GA {
    pub k: knapsack::Knapsack,
    pub n_individuals: usize,
    pub n_genes: usize,
    pub n_generations: usize,
    pub crossover_rate: f64,
    pub mutation_rate: f64,
    pub elitism_rate: f64
}

impl GA {
    fn generate_individual(&self) -> Individual {
        let mut g: Vec<bool> = Vec::with_capacity(self.n_genes);
        for _ in 0..self.n_genes {
            g.push(rand::random());
        }
        let f: f64 = self.k.objective(&g);
        return Individual {genes: g, fitness: f};
    }

    fn generate_population(&self) -> Vec<Individual> {
        let mut p: Vec<Individual> = Vec::with_capacity(self.n_individuals);
        for _ in 0..self.n_individuals {
            let ind: Individual = self.generate_individual();
            p.push(ind);
        }
        p.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        return p;
    }

    fn roulette_selection(&self, p: &[Individual]) -> Vec<Individual> {
        let mut skip: i64 = -1;
        let mut selected: Vec<Individual> = Vec::with_capacity(2);
        for i in 0..2 {
            let mut tf: f64 = 0.0;
            for j in 0..p.len() {
                if j as i64 == skip {
                    continue;
                }
                tf += p[j].fitness;
            }
            let mut cf: f64 = 0.0;
            let r: f64 = rand::random();
            for j in 0..p.len() {
                if j as i64 == skip {
                    continue;
                }
                if tf == 0.0 {
                    cf += 1.0 / (p.len() - i) as f64
                } else {
                    cf += p[j].fitness / tf;
                }
                if r < cf {
                    skip = j as i64;
                    selected.push(p[j].clone());
                    break;
                }
            }
        }
        return selected;
    }

    fn single_point_crossover(&self, selected: &mut Vec<Individual>) {
        let crossover_point: usize = rand::thread_rng().gen_range(1..self.n_genes);
        for i in 0..crossover_point {
            let temp: bool = selected[0].genes[i];
            selected[0].genes[i] = selected[1].genes[i];
            selected[1].genes[i] = temp;
        }
    }

    fn recombination(&self, p: &mut [Individual]) -> Vec<Individual> {
        let worst: f64 = p.last().unwrap().fitness;
        for i in 0..p.len() {
            p[i].fitness -= worst; 
        }

        let mut new_p: Vec<Individual> = Vec::with_capacity(self.n_individuals);

        for i in (0..self.n_individuals).step_by(2) {
            let mut selected: Vec<Individual> = self.roulette_selection(p);
            let r: f64 = rand::random();
            if r < self.crossover_rate {
                self.single_point_crossover(&mut selected);
            }
            for j in 0..2 {
                if self.n_individuals % 2 == 1 && i == self.n_individuals - 1 && j == 1 {
                    break;
                }
                new_p.push(selected[j].clone());
            }
        }
        return new_p;
    }

    fn mutation(&self, p: &mut [Individual]) {
        for i in 0..self.n_individuals {
            for j in 0..self.n_genes {
                let r: f64 = rand::random();
                if r < self.mutation_rate {
                    p[i].genes[j] = !p[i].genes[j];
                }
            }
        }
    }

    fn elitism(&self, p: &mut [Individual], new_p: &[Individual]) {
        let n_elites: usize = (self.elitism_rate * self.n_individuals as f64).ceil() as usize;
        for i in n_elites..self.n_individuals {
            p[i] = new_p[i - n_elites].clone();
        }
    }

    fn update_fitness(&self, p: &mut [Individual]) {
        for i in 0..self.n_individuals {
            p[i].fitness = self.k.objective(&p[i].genes);
        }
        p.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    }

    pub fn run(&self) -> Individual {
        let mut p: Vec<Individual> = self.generate_population();
        for _ in 1..self.n_generations + 1 {
            let mut new_p: Vec<Individual> = self.recombination(&mut p);
            self.mutation(&mut new_p);
            self.update_fitness(&mut new_p);
            self.elitism(&mut p, &new_p);
            self.update_fitness(&mut p);
        }
        return p.first().unwrap().clone();
    }
}
