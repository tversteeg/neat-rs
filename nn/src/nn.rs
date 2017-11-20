pub struct NeuralNetwork<'a> {
    data: Vec<f32>,
    activations: Vec<char>,

    pub inputs: &'a [f32],
    pub hiddens: &'a [f32],
    pub outputs: &'a [f32],

    pub bias: f32,
}

impl<'a> NeuralNetwork<'a> {
    pub fn new(
        inputAmount: i32,
        hiddenAmount: i32,
        outputAmount: i32,
        hiddenLayerAmount: i32,
    ) -> NeuralNetwork<'a> {
        let totalSize = inputAmount + (hiddenAmount * hiddenLayerAmount) + outputAmount;
        let activationSize = (hiddenAmount * hiddenLayerAmount) + outputAmount;

        NeuralNetwork {
            data: vec![0; totalSize as usize],
            activations: vec![0; activationSize as usize],

            bias: -1.0
        }
    }
}
