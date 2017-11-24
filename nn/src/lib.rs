extern crate rand;

pub mod nn;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn sanity() {
        let net = NeuralNetwork::new(0, 0, 0, 0);
    }

    #[test]
    fn run() {
        let net = NeuralNetwork::new(1, 1, 1, 1);
    }

    #[test]
    fn add_layer() {

    }
}
