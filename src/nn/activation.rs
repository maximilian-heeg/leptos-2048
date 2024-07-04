/// Computes the sigmoid activation function.
pub fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

/// Computes the rectified linear unit (ReLU) activation function.
pub fn relu(x: f64) -> f64 {
    if x > 0.0 {
        x
    } else {
        0.0
    }
}

/// Computes the hyperbolic tangent (tanh) activation function.
pub fn tanh(x: f64) -> f64 {
    x.tanh()
}

/// Enumeration of possible activation functions.
#[derive(Clone, Copy)]
pub enum ActivationFunction {
    Sigmoid,
    ReLU,
    Tanh,
    None,
}

impl ActivationFunction {
    /// Applies the activation function to the given input.
    pub fn activate(&self, x: f64) -> f64 {
        match self {
            ActivationFunction::Sigmoid => sigmoid(x),
            ActivationFunction::ReLU => relu(x),
            ActivationFunction::Tanh => tanh(x),
            ActivationFunction::None => x,
        }
    }
}
