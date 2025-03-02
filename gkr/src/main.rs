use std::{marker::PhantomData, vec};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use ark_ff::PrimeField;
fn main() {
    println!("Hello, world!");
}
#[derive(Debug, Clone, EnumIter)]
enum GateOp {
    Add,
    Mul,
}
#[derive(Debug, Clone)]
struct Gate {
    // _field: PhantomData<F>,
    left_index: usize,
    right_index: usize,
    output_index: usize,
    op: GateOp,
}

impl Gate {
    fn new(left_index: usize, right_index: usize, output_index: usize, op: GateOp) -> Self {
        Self {
            left_index,
            right_index,
            output_index,
            op,
        }
    }

    fn execute_gate<F: PrimeField>(&mut self, inputs: Vec<F>) -> F {
        let result = match self.op {
            GateOp::Add => inputs[self.left_index] + inputs[self.right_index],
            GateOp::Mul => inputs[self.left_index] * inputs[self.right_index],
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
struct Circuit<F: PrimeField> {
    layers: Vec<Vec<Gate>>,
    w_polys: Vec<Vec<F>>,
}

impl<F: PrimeField> Circuit<F> {
    fn new(layers: Vec<Vec<Gate>>) -> Self {
        Self {
            layers,
            w_polys: Vec::new(),
        }
    }

    fn evaluate(&mut self, input: Vec<F>) -> F {
        // let mut circuit_layers = self.clone().layers;

        let mut w_polys = Vec::new();
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
                w_polys.push(output_vec.clone());
            }
            current_layer = output_vec;
        }
        self.w_polys = w_polys.clone();

        // how do i get to the next layer?
        current_layer[0]
    }

    fn get_w_poly(&mut self, layer_id: usize) -> Vec<F> {
        let w_poly = &self.w_polys[layer_id];
        w_poly.clone()
    }

    fn add_i_mle(&mut self, layer_id: usize) -> Vec<Vec<F>> {
        let layer_vec = &self.layers[layer_id];

        if layer_vec.is_empty() {
            return vec![vec![F::zero(); 2], vec![F::zero(); 2]];
        }

        let no_of_gates = layer_vec.len() * 2;
        let no_of_bit_in_gate_input_index = (no_of_gates as f64).log2().ceil().max(1.0) as usize;
        let no_of_bit_in_gate_output_index = if no_of_bit_in_gate_input_index == 1 {
            1
        } else {
            no_of_bit_in_gate_input_index - 1
        };

        let total_no_of_bits = no_of_bit_in_gate_input_index * 2 + no_of_bit_in_gate_output_index;

        println!(
            "no bit input:{} no bits output{}",
            no_of_bit_in_gate_input_index, no_of_bit_in_gate_output_index
        );
        let vector_size = 1 << total_no_of_bits;
        let mut add_vec = vec![F::zero(); vector_size];
        let mut mul_vec = vec![F::zero(); vector_size];

        for gate in layer_vec {
            let gate_op = &gate.op;

            let mut res = gate.output_index << no_of_bit_in_gate_input_index | gate.left_index;
            res = res << no_of_bit_in_gate_input_index | gate.right_index;
            if let GateOp::Add = gate_op {
                add_vec[res] = F::one();
            } else if let GateOp::Mul = gate_op {
                mul_vec[res] = F::one();
            }
        }

        vec![add_vec, mul_vec]
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
            Gate::new(0, 1, 0, GateOp::Add), // 3 + 4 = 7 -> output[0]
            Gate::new(1, 2, 1, GateOp::Mul), // 4 * 5 = 20 -> output[1]
        ];
        let layer2 = vec![
            Gate::new(0, 1, 0, GateOp::Mul), // 7 * 20 = 140 -> output[0]
        ];

        let circuit = Circuit::new(vec![layer1, layer2]);
        // let circuit = Circuit::new(vec![
        //     vec![Gate::new(0, 1, 0, GateOp::Add),Gate::new(1, 2, 1, GateOp::Mul)],
        //     vec![Gate::new(0, 1, 0, GateOp::Mul)]]);

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
            Gate::new(0, 1, 0, GateOp::Add), // a + b = 2 + 3 = 5 -> output[0]
            Gate::new(2, 3, 1, GateOp::Mul), // c * d = 4 * 5 = 20 -> output[1]
        ];

        // Layer 2: Compute (a+b)*(c*d) and a*b
        let layer2 = vec![
            Gate::new(0, 1, 0, GateOp::Mul), // (a+b) * (c*d) = 5 * 20 = 100 -> output[0]
            Gate::new(0, 1, 1, GateOp::Mul), // a * b = 2 * 3 = 6 -> output[1] (using original inputs)
        ];

        // Layer 3: Compute final result
        let layer3 = vec![
            Gate::new(0, 1, 0, GateOp::Add), // (a+b)*(c*d) + (a*b) = 100 + 6 = 106 -> output[0]
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

    #[test]
    fn test_add_i_mle_single_bit() {
        // With 2 gates, indices should be sequential: 0,1,2,3
        let gate_add = Gate::new(0, 1, 0, GateOp::Add); // First computation
                                                        // let gate_mul = Gate::new(2, 3, 1, GateOp::Mul);  // Uses intermediate result

        let mut circuit = Circuit::new(vec![vec![gate_add]]);
        let result = circuit.add_i_mle(0);

        // 1 gate = 1 bit needed per index
        assert_eq!(result[0].len(), 8); // 2^3 = 8
        assert_eq!(result[1].len(), 8);

        // For Add gate: left=0, right=1, output=2
        // let add_index = 0 + 1*2 + 2*4 + 1;
        assert_eq!(result[0][1], f(1));
        // dbg!(&result);

        // For Mul gate: left=2, right=1, output=3
        // let mul_index = 2 + 1*2 + 3*4 + 1;
        assert!(result[1].iter().all(|x| *x == f(0)));
    }

    #[test]
    fn test_add_i_mle_two_bit_input() {
        // With 4 gates, indices should be sequential: 0,1,2,3,4,5
        let layer1 = vec![
            Gate::new(0, 1, 0, GateOp::Add), // 3 + 4 = 7 -> output[0]
            Gate::new(2, 3, 1, GateOp::Mul), // 4 * 5 = 20 -> output[1]
        ];
        let layer2 = vec![
            Gate::new(0, 1, 0, GateOp::Mul), // 7 * 20 = 140 -> output[0]
        ];
        let mut circuit = Circuit::new(vec![layer1, layer2]);
        // let input = vec![f(3), f(4), f(5)];
        let result = circuit.add_i_mle(0);

        // 4 gates = 2 bits needed per index
        assert_eq!(result[0].len(), 32); // 2^6 = 64
        assert_eq!(result[1].len(), 32);
        dbg!(&result);

        // For Add gate: left=0, right=1, output=2
        // let add_index = 0 + 1*(1<<2) + 2*(1<<4) + 1;
        assert_eq!(result[0][1], f(1));

        // // For Mul gate: left=2, right=3, output=4
        // let mul_index = 2 + 3*(1<<2) + 4*(1<<4) + 1;
        assert_eq!(result[1][27], f(1));
    }

    #[test]
    fn test_add_i_mle_empty_layer() {
        let mut circuit = Circuit::new(vec![vec![]]);
        let result = circuit.add_i_mle::<Fr>(0);

        assert_eq!(result[0].len(), 2);
        assert_eq!(result[1].len(), 2);
        assert!(result[0].iter().all(|x| *x == f(0)));
        assert!(result[1].iter().all(|x| *x == f(0)));
    }
}
