extern crate rand;

use rand::Rng;

#[derive(Debug)]
pub struct NeuralNetwork {
    data: Vec<f32>,
    activations: Vec<u8>,

    inputs: i32,
    hiddens: i32,
    outputs: i32,
    hidden_layers: i32,

    pub bias: f32,
}

impl NeuralNetwork {
    pub fn new(
        input_amount: i32,
        hidden_amount: i32,
        output_amount: i32,
        hidden_layer_amount: i32,
    ) -> NeuralNetwork {
        let total_size = input_amount + (hidden_amount * hidden_layer_amount) + output_amount;
        let activation_size = (hidden_amount * hidden_layer_amount) + output_amount;

        NeuralNetwork {
            data: vec![0.0; total_size as usize],
            activations: vec![0; activation_size as usize],

            inputs: input_amount,
            hiddens: hidden_amount,
            outputs: output_amount,
            hidden_layers: hidden_layer_amount,

            bias: -1.0
        }
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();

        for node in self.data.iter_mut() {
            *node = rng.gen::<f32>();
        }
    }
}
