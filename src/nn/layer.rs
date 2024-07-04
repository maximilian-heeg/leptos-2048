use super::activation::ActivationFunction;
use super::node::Node;

/// A layer in a neural network, consisting of multiple nodes.
#[derive(Clone)]
pub struct Layer {
    pub nodes: Vec<Node>,
    pub activation_function: ActivationFunction,
}

impl Layer {
    /// Creates a new layer with the given number of
    /// input and output nodes, and an activation function.
    ///
    /// # Arguments
    ///
    /// * `input_size` - The number of inputs to each node in the layer.
    /// * `output_size` - The number of nodes in the layer.
    /// * `activation_function` - The activation function
    ///   to apply to the output of each node.
    ///
    /// # Returns
    ///
    /// A new `Layer` instance.
    pub fn new(
        input_size: usize,
        output_size: usize,
        activation_function: ActivationFunction,
    ) -> Self {
        let nodes = (0..output_size).map(|_| Node::new(input_size)).collect();

        Layer {
            nodes,
            activation_function,
        }
    }

    /// Performs a forward pass through the layer by
    /// computing the output of each node.
    ///
    /// # Arguments
    ///
    /// * `inputs` - A vector of input values to the layer.
    ///
    /// # Returns
    ///
    /// A vector of output values from the layer.
    pub fn forward(&self, inputs: Vec<f64>) -> Vec<f64> {
        self.nodes
            .iter()
            .map(|node| {
                let sum = node.forward(&inputs);
                self.activation_function.activate(sum)
            })
            .collect()
    }

    /// Updates the nodes in the layer by random changes.
    ///
    /// # Arguments
    ///
    /// * `rate` - The probability of each weight (and the bias) being updated.
    /// * `variation` - The variation at which the weights/parameters change.
    pub fn update(&mut self, rate: f64, variation: f64) {
        for node in &mut self.nodes {
            node.update(rate, variation);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::activation::ActivationFunction;
    use super::super::node::Node;
    use super::*;

    #[test]
    fn test_new_layer() {
        let input_size = 2;
        let output_size = 3;
        let activation_function = ActivationFunction::Sigmoid;
        let layer = Layer::new(input_size, output_size, activation_function);

        assert_eq!(layer.nodes.len(), output_size);
        for node in &layer.nodes {
            assert_eq!(node.weights.len(), input_size);
        }
    }

    #[test]
    fn test_forward() {
        let layer = Layer {
            nodes: vec![
                Node {
                    weights: vec![0.5, 0.5],
                    bias: 0.0,
                },
                Node {
                    weights: vec![0.3, 0.3],
                    bias: 0.0,
                },
            ],
            activation_function: ActivationFunction::Sigmoid,
        };

        let inputs = vec![1.0, 1.0];
        let outputs = layer.forward(inputs);

        let expected_output_1 = 1.0 / (1.0 + (-1.0f64).exp()); // sigmoid(0.5 * 1 + 0.5 * 1 + 0)
        let expected_output_2 = 1.0 / (1.0 + (-0.6f64).exp()); // sigmoid(0.3 * 1 + 0.3 * 1 + 0)
        assert_eq!(outputs.len(), 2);
        assert!((outputs[0] - expected_output_1).abs() < 1e-6);
        assert!((outputs[1] - expected_output_2).abs() < 1e-6);
    }
}
