use rand;
use rand::distributions::{IndependentSample, Range};

#[derive(Debug, Copy, Clone)]
pub enum NeuralActivation {
    Passthrough,
    Sigmoid,
    FastSigmoid,
    Relu
}

#[derive(Debug, Copy, Clone)]
pub struct NeuralNode {
    result: f32,
    activation: NeuralActivation
}

impl Default for NeuralNode {
    fn default() -> NeuralNode {
        NeuralNode {
            result: 0.0,
            activation: NeuralActivation::Passthrough
        }
    }
}

#[derive(Debug)]
pub struct NeuralNetwork {
    nodes: Vec<NeuralNode>,
    weights: Vec<f32>,

    pub bias: f32,
}

impl NeuralNetwork {
    pub fn new(
        input_amount: usize,
        hidden_amount: usize,
        output_amount: usize,
        hidden_layer_amount: usize,
    ) -> NeuralNetwork {
        if input_amount == 0 {
            panic!("Amount of network input nodes needs to be bigger than 0.")
        }
        if hidden_amount == 0 {
            panic!("Amount of network hidden nodes needs to be bigger than 0.")
        }
        if output_amount == 0 {
            panic!("Amount of network output nodes needs to be bigger than 0.")
        }

        NeuralNetwork {
            nodes: vec![Default::default(); NeuralNetwork::node_len(input_amount, hidden_amount, output_amount, hidden_layer_amount)],
            weights: vec![0.0; NeuralNetwork::weight_len(input_amount, hidden_amount, output_amount, hidden_layer_amount)],

            bias: -1.0
        }
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        let range = Range::new(-2.0, 2.0);

        for weight in self.weights.iter_mut() {
            *weight = range.ind_sample(&mut rng);
        }
    }

    pub fn run(&mut self, inputs: &[f32]) {
        // Copy inputs to the output space
        for i in 0 .. inputs.len() {
            
        }
    }

    fn node_len(inputs: usize, hiddens: usize, outputs: usize, hidden_layers: usize) -> usize {
        inputs + hiddens * hidden_layers + outputs
    }

    fn weight_len(inputs: usize, hiddens: usize, outputs: usize, hidden_layers: usize) -> usize {
        let input_weights = (inputs + 1) * hiddens;
        let hidden_weights = hidden_layers * (hiddens + 1) * hiddens;
        let output_weights = if hidden_layers == 0 { hiddens + 1 } else { inputs + 1 } * outputs;

        input_weights + hidden_weights + output_weights
    }
}
