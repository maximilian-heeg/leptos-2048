use rand::Rng;

/// A single node (neuron) in a neural network layer.
#[derive(Clone)]
pub struct Node {
    pub weights: Vec<f64>,
    pub bias: f64,
}

impl Node {
    /// Creates a new node with random weights and bias.
    ///
    /// # Arguments
    ///
    /// * `input_size` - The number of inputs to this node.
    ///
    /// # Returns
    ///
    /// A new `Node` instance with random weights and bias.
    pub fn new(input_size: usize) -> Self {
        let mut rng = rand::thread_rng();

        let weights = (0..input_size).map(|_| rng.gen::<f64>()).collect();

        let bias = rng.gen::<f64>();

        Node { weights, bias }
    }

    /// Performs a forward pass by computing the weighted sum of the inputs and adding the bias.
    ///
    /// # Arguments
    ///
    /// * `inputs` - A slice of input values.
    ///
    /// # Returns
    ///
    /// The weighted sum of the inputs plus the bias.
    pub fn forward(&self, inputs: &[f64]) -> f64 {
        assert!(self.weights.len() == inputs.len());
        self.weights
            .iter()
            .zip(inputs)
            .map(|(w, i)| w * i)
            .sum::<f64>()
            + self.bias
    }

    /// Updates the weights and bias of the node by random changes.
    ///
    /// # Arguments
    ///
    /// * `rate` - The probability of each weight (and the bias) being updated.
    /// * `variation` - The variation at which the weights/parameters change.
    pub fn update(&mut self, rate: f64, variation: f64) {
        let mut rng = rand::thread_rng();
        let num_weights = self.weights.len();

        for index in 0..num_weights {
            if rng.gen::<f64>() < rate {
                let change = rng.gen_range(-variation..variation); // random value in range [-1, 1]
                self.weights[index] += change;
            }
        }

        if rng.gen::<f64>() < rate {
            let bias_change = rng.gen_range(-variation..variation); // random value in range [-1, 1]
            self.bias += bias_change;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_node() {
        let input_size = 3;
        let node = Node::new(input_size);

        assert_eq!(node.weights.len(), input_size);
        // Check if weights are within expected range
        for &weight in &node.weights {
            assert!(weight >= 0.0 && weight <= 1.0, "Weight out of range");
        }
        assert!(node.bias >= 0.0 && node.bias <= 1.0, "Bias out of range");
    }

    #[test]
    fn test_forward() {
        let node = Node {
            weights: vec![0.5, 0.5],
            bias: 1.0,
        };

        let inputs = vec![1.0, 1.0];
        let output = node.forward(&inputs);

        assert_eq!(output, 2.0);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_forward_panic_on_mismatched_inputs() {
        let node = Node {
            weights: vec![0.5, 0.5],
            bias: 1.0,
        };

        let inputs = vec![1.0];
        node.forward(&inputs); // This should panic due to mismatched input size
    }

    #[test]
    fn test_update_with_no_changes() {
        let mut node = Node::new(5);
        let original_weights = node.weights.clone();
        let original_bias = node.bias;

        // Use rate 0 to ensure no updates
        node.update(0.0, 1.0);

        assert_eq!(original_weights, node.weights);
        assert_eq!(original_bias, node.bias);
    }

    #[test]
    fn test_update_with_all_changes() {
        let mut node = Node::new(5);
        let original_weights = node.weights.clone();
        let original_bias = node.bias;

        // Use rate 1 to ensure all weights and bias are updated
        node.update(1.0, 0.1);

        let num_changes = original_weights
            .iter()
            .zip(node.weights.iter())
            .filter(|&(a, b)| (a - b).abs() > f64::EPSILON)
            .count();

        // All weights should be changed
        assert_eq!(num_changes, node.weights.len());

        // Bias should be changed
        assert!((original_bias - node.bias).abs() > f64::EPSILON);
    }

    #[test]
    fn test_update_variation_range() {
        let mut node = Node::new(5);
        let original_weights = node.weights.clone();

        // Use a high rate to ensure updates and a large variation
        let variation = 0.5;
        node.update(1.0, variation);

        for (original, updated) in original_weights.iter().zip(node.weights.iter()) {
            let diff = (original - updated).abs();
            assert!(diff <= variation); // Check if the change is within the variation range
        }
    }
}
