pub mod activation;
mod layer;
mod node;
use std::fs::File;
use std::io::{self};

use activation::ActivationFunction;
use layer::Layer;
use serde::{Deserialize, Serialize};

/// A neural network consisting of multiple layers.
#[derive(Clone, Serialize, Deserialize)]
pub struct NeuralNetwork {
    pub layers: Vec<Layer>,
}

impl NeuralNetwork {
    /// Creates a new neural network with the given
    /// layer sizes and activation functions.
    ///
    /// # Arguments
    ///
    /// * `layer_sizes` - A slice of the sizes of each layer.
    /// * `activation_functions` - A slice of activation
    ///    functions for each layer.
    ///
    /// # Returns
    ///
    /// A new `NeuralNetwork` instance.
    pub fn new(layer_sizes: &[usize], activation_functions: &[ActivationFunction]) -> Self {
        assert!(layer_sizes.len() == activation_functions.len() + 1);

        let mut layers = Vec::new();

        for i in 0..layer_sizes.len() - 1 {
            layers.push(Layer::new(
                layer_sizes[i],
                layer_sizes[i + 1],
                activation_functions[i],
            ));
        }

        NeuralNetwork { layers }
    }

    /// Performs a forward pass through the neural network
    /// by computing the output of each layer.
    ///
    /// # Arguments
    ///
    /// * `inputs` - A vector of input values to the neural network.
    ///
    /// # Returns
    ///
    /// A vector of output values from the neural network.
    pub fn forward(&self, inputs: Vec<f64>) -> Vec<f64> {
        let mut output = inputs;
        for layer in &self.layers {
            output = layer.forward(output);
        }
        output
    }

    /// Updates the layers in the neural network by random changes.
    ///
    /// # Arguments
    ///
    /// * `rate` - The probability of each weight (and the bias) being updated.
    /// * `variation` - The variation at which the weights/parameters change.
    pub fn update(&mut self, rate: f64, variation: f64) {
        for layer in &mut self.layers {
            layer.update(rate, variation);
        }
    }

    /// Saves the neural network to a file in JSON format.
    ///
    /// # Arguments
    ///
    /// * `filename` - The name of the file to save the neural network to.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure.
    pub fn save(&self, filename: &str) -> io::Result<()> {
        let file = File::create(filename)?;
        serde_json::to_writer(file, &self)?;
        Ok(())
    }

    /// Loads a neural network from a file in JSON format.
    ///
    /// # Arguments
    ///
    /// * `filename` - The name of the file to load the neural network from.
    ///
    /// # Returns
    ///
    /// A `Result` containing the loaded `NeuralNetwork` or an error.
    pub fn load(filename: &str) -> io::Result<Self> {
        let file = File::open(filename)?;
        let network = serde_json::from_reader(file)?;
        Ok(network)
    }
}

#[cfg(test)]
mod tests {
    use super::activation::ActivationFunction;
    use super::*;
    use std::fs;

    #[test]
    fn test_new_neural_network() {
        let layer_sizes = &[2, 3, 1];
        let activation_functions = vec![ActivationFunction::Sigmoid, ActivationFunction::Sigmoid];

        let nn = NeuralNetwork::new(layer_sizes, &activation_functions);

        assert_eq!(nn.layers.len(), 2);
        assert_eq!(nn.layers[0].nodes.len(), 3);
        assert_eq!(nn.layers[1].nodes.len(), 1);
    }

    #[test]
    fn test_forward() {
        let layer_sizes = &[2, 2, 1];
        let activation_functions = vec![ActivationFunction::Sigmoid, ActivationFunction::Sigmoid];

        let mut nn = NeuralNetwork::new(layer_sizes, &activation_functions);

        // Manually setting weights and biases for deterministic testing
        nn.layers[0].nodes[0].weights = vec![0.5, 0.5];
        nn.layers[0].nodes[0].bias = 0.0;
        nn.layers[0].nodes[1].weights = vec![0.3, 0.3];
        nn.layers[0].nodes[1].bias = 0.0;
        nn.layers[1].nodes[0].weights = vec![0.2, 0.2];
        nn.layers[1].nodes[0].bias = 0.0;

        let inputs = vec![1.0, 1.0];
        let outputs = nn.forward(inputs);

        let expected_output_0 = 1.0 / (1.0 + (-1.0f64).exp()); // sigmoid(0.5 * 1 + 0.5 * 1 + 0)
        let expected_output_1 = 1.0 / (1.0 + (-0.6f64).exp()); // sigmoid(0.3 * 1 + 0.3 * 1 + 0)
        let expected_final_output =
            1.0 / (1.0 + (-(0.2 * expected_output_0 + 0.2 * expected_output_1)).exp()); // sigmoid(0.2 * output_0 + 0.2 * output_1)

        assert_eq!(outputs.len(), 1);
        assert!((outputs[0] - expected_final_output).abs() < 1e-6);
    }

    #[test]
    fn test_save_and_load() {
        let layer_sizes = vec![2, 3, 1];
        let activation_functions = vec![ActivationFunction::ReLU, ActivationFunction::Sigmoid];
        let network = NeuralNetwork::new(&layer_sizes, &activation_functions);

        let filename = "test_model.json";
        network.save(filename).expect("Failed to save the network");

        let loaded_network = NeuralNetwork::load(filename).expect("Failed to load the network");

        assert_eq!(network.layers.len(), loaded_network.layers.len());

        // Clean up
        fs::remove_file(filename).expect("Failed to remove test file");
    }
}
