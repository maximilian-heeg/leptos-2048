use crate::game::Actions;
use crate::game::Game;
use crate::nn::NeuralNetwork;
use itertools::Itertools;

#[derive(Clone)]
pub struct Agent {
    pub nn: NeuralNetwork,
    pub scores: Vec<u32>,
    pub highest_tiles: Vec<u32>,
    pub game: Game,
    pub steps: usize,
}

impl Agent {
    pub fn new(nn: NeuralNetwork, game: Game) -> Self {
        Self {
            nn,
            game,
            scores: vec![],
            highest_tiles: vec![],
            steps: 0,
        }
    }

    pub fn predict(&self) -> Actions {
        let output = self.nn.forward(self.game.flatten());
        assert!(output.len() == 4);
        let arg_max = output
            .iter()
            .position_max_by(|x, y| x.total_cmp(y))
            .expect("Cannot find maximal value");
        Actions::try_from(arg_max).expect("Cannot map integer to action")
    }

    pub fn step(&mut self) -> bool {
        if self.game.is_game_over() {
            return false;
        }
        let changed = self.game.step(self.predict());
        self.steps += 1;
        changed
    }

    pub fn play(&mut self, max_steps: usize) {
        while !self.game.is_game_over() && self.steps < max_steps {
            let changed = self.step();
            if !changed {
                break;
            }
        }
        self.scores.push(self.game.score);
        self.highest_tiles.push(
            self.game
                .highest_tile()
                .expect("Could not get highest tile"),
        );
    }

    pub fn reset(&mut self) {
        self.steps = 0;
        self.game = Game::new();
    }

    pub fn get_highest_tile(&self) -> Option<&u32> {
        let arg_max = self
            .highest_tiles
            .iter()
            .position_max()
            .expect("Cannot find maximal value");
        self.highest_tiles.get(arg_max)
    }

    pub fn avg_score(&self) -> f64 {
        self.scores.iter().sum::<u32>() as f64 / self.scores.len() as f64
    }

    pub fn mutate(&mut self, rate: f64, variation: f64) {
        self.nn.update(rate, variation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::*;
    use crate::nn::activation::ActivationFunction;
    use crate::nn::NeuralNetwork;

    #[test]
    fn test_2048_nn() {
        let game = Game::new();
        let nn = NeuralNetwork::new(&[16, 16, 8, 4], &[ActivationFunction::None; 3]);
        let input = game.flatten();
        let output = nn.forward(input);
        assert!(output.len() == 4);
    }

    #[test]
    fn test_agent() {
        let nn = NeuralNetwork::new(&[16, 16, 8, 4], &[ActivationFunction::None; 3]);
        let game = Game::new();
        let mut a = Agent::new(nn, game);
        a.step();
        assert!(a.steps == 1)
    }
}
