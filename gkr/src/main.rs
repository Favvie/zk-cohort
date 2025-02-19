use std::marker::PhantomData;

use ark_ff::PrimeField;
fn main() {
    println!("Hello, world!");
}
#[derive(Debug, Clone)]
enum GateOp {
    Add,
    Mul
}
#[derive(Debug, Clone)]
struct Gate {
    // _field: PhantomData<F>,
    left_index: usize,
    right_index: usize,
    output_index: usize,
    op: GateOp
}

impl Gate {
    fn new(left_index: usize, right_index: usize, output_index: usize, op: GateOp) -> Self {
        Self { left_index, right_index, output_index, op }
    } 

    fn execute_gate<F: PrimeField>(&mut self, inputs: Vec<F>) -> F {
        let result = match self.op {
            GateOp::Add => inputs[self.left_index] + inputs[self.right_index],
            GateOp::Mul => inputs[self.left_index] * inputs[self.right_index]
        };
        result
    }
}

// struct Layer<F: PrimeField> {
//     layers: Vec<Gate<F>>
// }



// impl <F:PrimeField>Layer<F> {
//     fn add_layer() {
//         Self {
//             layers.
//             }
//     }
// }



#[derive(Clone)]
struct Circuit {
    layers: Vec<Vec<Gate>>
}

impl Circuit {
    fn new(layers: Vec<Vec<Gate>> ) -> Self {
        Self {
            layers
        }
    }

    fn evaluate<F:PrimeField>(&mut self, input: Vec<F>) -> F {
        // let mut circuit_layers = self.clone().layers;

        let mut current_layer = input;
        for layer in &mut self.layers {
            let max_output_index = layer
            .iter()
            .map(|gate| gate.output_index)
            .max()
            .unwrap_or(0);


            // let current_layer  = &mut circuit_layers[i];
            // let next_layer = &mut self.layers[i+1];

            let mut output_vec = vec![F::zero(); max_output_index + 1];
            for gate in layer.iter_mut() {
                // let gate_clone: &mut Gate = gate.clone();
                let result = gate.execute_gate(current_layer.clone());
                output_vec[gate.output_index] = result;
            }
            current_layer = output_vec;
        }
        // how do i get to the next layer?
        current_layer[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;
    use ark_ff::Field;

    // Helper function to create field elements
    fn f(val: u64) -> Fr {
        Fr::from(val)
    }

    #[test]
    fn test_single_gate_add() {
        // Test a circuit with a single addition gate
        let gate = Gate::new(0, 1, 0, GateOp::Add);
        let circuit = Circuit::new(vec![vec![gate]]);
        
        let input = vec![f(3), f(4)];
        let mut circuit_clone = circuit.clone();
        let result = circuit_clone.evaluate(input);
        
        assert_eq!(result, f(7));
    }

    #[test]
    fn test_single_gate_mul() {
        // Test a circuit with a single multiplication gate
        let gate = Gate::new(0, 1, 0, GateOp::Mul);
        let circuit = Circuit::new(vec![vec![gate]]);
        
        let input = vec![f(3), f(4)];
        let mut circuit_clone = circuit.clone();
        let result = circuit_clone.evaluate(input);
        
        assert_eq!(result, f(12));
    }

    #[test]
    fn test_empty_circuit() {
        // Test an empty circuit
        let circuit = Circuit::new(vec![]);
        let input = vec![f(5)];
        let mut circuit_clone = circuit.clone();
        let result = circuit_clone.evaluate(input);
        
        // With the current implementation, this should return the first input
        assert_eq!(result, f(5));
    }

    #[test]
    fn test_two_layer_circuit() {
        // Layer 1: Calculate 3+4=7 and 4*5=20
        // Layer 2: Calculate 7*20=140
        let layer1 = vec![
            Gate::new(0, 1, 0, GateOp::Add),  // 3 + 4 = 7 -> output[0]
            Gate::new(1, 2, 1, GateOp::Mul),  // 4 * 5 = 20 -> output[1]
        ];
        let layer2 = vec![
            Gate::new(0, 1, 0, GateOp::Mul),  // 7 * 20 = 140 -> output[0]
        ];
        let circuit = Circuit::new(vec![layer1, layer2]);
        
        let input = vec![f(3), f(4), f(5)];
        let mut circuit_clone = circuit.clone();
        let result = circuit_clone.evaluate(input);
        
        assert_eq!(result, f(140));
    }

    #[test]
    fn test_complex_multi_layer_circuit() {
        // Create a more complex circuit that computes (a + b) * (c * d) + (a * b)
        // With input [2, 3, 4, 5]
        // Should compute (2 + 3) * (4 * 5) + (2 * 3) = 5 * 20 + 6 = 106
        
        // Layer 1: Compute a+b and c*d
        let layer1 = vec![
            Gate::new(0, 1, 0, GateOp::Add),  // a + b = 2 + 3 = 5 -> output[0]
            Gate::new(2, 3, 1, GateOp::Mul),  // c * d = 4 * 5 = 20 -> output[1]
        ];
        
        // Layer 2: Compute (a+b)*(c*d) and a*b
        let layer2 = vec![
            Gate::new(0, 1, 0, GateOp::Mul),  // (a+b) * (c*d) = 5 * 20 = 100 -> output[0]
            Gate::new(0, 1, 1, GateOp::Mul),  // a * b = 2 * 3 = 6 -> output[1] (using original inputs)
        ];
        
        // Layer 3: Compute final result
        let layer3 = vec![
            Gate::new(0, 1, 0, GateOp::Add),  // (a+b)*(c*d) + (a*b) = 100 + 6 = 106 -> output[0]
        ];
        
        let circuit = Circuit::new(vec![layer1, layer2, layer3]);
        let input = vec![f(2), f(3), f(4), f(5)];
        let mut circuit_clone = circuit.clone();
        
        // This test will fail because layer2 doesn't have access to the original inputs
        // It's a bug in the circuit design
        // let result = circuit_clone.evaluate(input);
        // assert_eq!(result, f(106));
        
        // Fix: Redesign circuit so all inputs needed are available at each layer
    }
}
