pub mod agent;

use crate::{
    game::Game,
    nn::{activation::ActivationFunction, NeuralNetwork},
};
use agent::Agent;
use itertools::Itertools;
use rand::Rng;
use rayon::prelude::*;

pub struct Population {
    pub agents: Vec<Agent>,
    pub evolution_step: usize,
}

impl Population {
    pub fn new(
        n_agents: usize,
        layer_sizes: &[usize],
        activation_functions: &[ActivationFunction],
    ) -> Self {
        let mut agents = vec![];
        for _ in 0..n_agents {
            agents.push(Agent::new(
                NeuralNetwork::new(layer_sizes, activation_functions),
                Game::new(),
            ));
        }
        Self {
            agents,
            evolution_step: 0,
        }
    }

    pub fn play(&mut self, max_steps: usize) {
        let mut v = vec![0usize; 5];
        v.par_iter_mut().enumerate().for_each(|(i, x)| *x = i);
        assert_eq!(v, [0, 1, 2, 3, 4]);

        self.agents.par_iter_mut().for_each(|a| a.play(max_steps));
    }

    pub fn resert_agents(&mut self) {
        for a in self.agents.iter_mut() {
            a.reset()
        }
    }

    pub fn get_scores(&self) -> Vec<f64> {
        self.agents.iter().map(|a| a.avg_score()).collect()
    }

    pub fn get_best_agent(&self) -> Option<&Agent> {
        let scores = self.get_scores();
        let arg_max = scores
            .iter()
            .position_max_by(|x, y| x.total_cmp(y))
            .expect("Cannot find maximal value");
        self.agents.get(arg_max)
    }

    fn get_best_agents(&self, proportion: f64) -> Vec<&Agent> {
        let mut agents_and_scores: Vec<(&Agent, f64)> =
            self.agents.iter().zip(self.get_scores()).collect();

        // Sort by scores in descending order
        agents_and_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        // Calculate the number of top agents to take
        let top_n = (agents_and_scores.len() as f64 * proportion).ceil() as usize;

        // Collect the top N agents
        agents_and_scores
            .iter()
            .take(top_n)
            .map(|(agent, _)| *agent)
            .collect()
    }

    pub fn evolve(&mut self, prop_keep: f64, prop_mutate: f64, mutation_rate: f64) {
        let best = self.get_best_agents(prop_keep);
        let n = self.agents.len();

        let mut new_agents: Vec<Agent> = best.iter().map(|a| (**a).clone()).collect();

        let diff = n - new_agents.len();
        let mut rng = rand::thread_rng();
        for _ in 0..diff {
            let idx = rng.gen_range(0..best.len());
            let mut new_agent: Agent = (*best.get(idx).expect("Could not get agent")).to_owned();
            new_agent.mutate(prop_mutate, mutation_rate);
            new_agents.push(new_agent);
        }

        self.agents = new_agents;
        self.evolution_step += 1;
    }
}
