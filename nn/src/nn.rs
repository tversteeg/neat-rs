use rand;
use rand::distributions::{IndependentSample, Range};

#[derive(Debug, Copy, Clone)]
pub struct NeuralNode {
    weight: f32,
    activation: u8
}

impl NeuralNode {
    pub fn new() -> NeuralNode {
        NeuralNode {
            weight: 0.0,
            activation: 0
        }
    }
}

#[derive(Debug)]
pub struct NeuralNetwork {
    inputs: Vec<NeuralNode>,
    hiddens: Vec<Vec<NeuralNode>>,
    outputs: Vec<NeuralNode>,

    pub bias: f32,
}

impl NeuralNetwork {
    pub fn new(
        input_amount: usize,
        hidden_amount: usize,
        output_amount: usize,
        hidden_layer_amount: usize,
    ) -> NeuralNetwork {
        let node = NeuralNode::new();

        let hidden_layer = vec![node; hidden_amount];

        NeuralNetwork {
            inputs: vec![node; input_amount],
            hiddens: vec![hidden_layer; hidden_layer_amount],
            outputs: vec![node; output_amount],

            bias: -1.0
        }
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        let range = Range::new(-2.0, 2.0);

        for node in self.inputs.iter_mut() {
            node.weight = range.ind_sample(&mut rng);
        }

        for layer in self.hiddens.iter_mut() {
            for node in layer.iter_mut() {
                node.weight = range.ind_sample(&mut rng);
            }
        }

        for node in self.outputs.iter_mut() {
            node.weight = range.ind_sample(&mut rng);
        }
    }

    pub fn run(&mut self, inputs: &[f32]) {
        // Copy inputs to the output space
        for i in 0 .. inputs.len() {
            
        }
    }
}
