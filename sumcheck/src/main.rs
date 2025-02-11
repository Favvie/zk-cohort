use ark_ff::{BigInteger, PrimeField};
mod transcript;
use transcript::{Transcript, Keccak256Hasher};
use sha3::{Digest, Keccak256};

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone)]
struct UnivariatePoly<F: PrimeField> {
    evals: Vec<F>,
}

#[derive(Debug, Clone)]
struct Proof<F: PrimeField> {
    initial_claimed_sum: F,
    proof: Vec<UnivariatePoly<F>>,
}

struct Prover<F: PrimeField> {
    initial_poly: Vec<F>,
    n_vars: usize,
}

struct Verifier<F: PrimeField> {
    initial_poly: Vec<F>,
    n_vars: usize,
}
impl <F:PrimeField>Prover<F> {
    fn new(n_vars: usize, initial_poly: Vec<F>) -> Self {
        assert_eq!(initial_poly.len(), 1<< n_vars, "Initial polynomial length must be 2^n_vars");
        Self {
            initial_poly,
            n_vars
        }
    }

    fn prove(&self) -> Proof<F> {
        let hasher = Keccak256Hasher::new();
        let mut transcript: Transcript<Keccak256Hasher, F> = Transcript::init(hasher);
        
        // Absorb initial polynomial
        let bytes = to_bytes(self.initial_poly.clone());
        transcript.absorb(&bytes);
        
        // Calculate initial sum
        let mut initial_claimed_sum = F::zero();
        for coeff in &self.initial_poly {
            initial_claimed_sum += coeff;
        }
        // Absorb claimed sum
        transcript.absorb(&to_bytes(vec![initial_claimed_sum]));
        
        // Create first univariate polynomial
        let (y0, y1) = self.create_paired_arrays(0);
        
        // Calculate evaluations at x = 0 and x = 1
        let mut eval_0 = F::zero();
        let mut eval_1 = F::zero();
        
        // Properly sum up the evaluations
        for i in 0..y0.len() {
            eval_0 += y0[i];
            eval_1 += y1[i];
        }
        
        let univariate_evals = vec![eval_0, eval_1];
        
        // Absorb univariate polynomial
        transcript.absorb(&to_bytes(univariate_evals.clone()));
        
        let mut univariate_polys = vec![UnivariatePoly { evals: univariate_evals }];
        let mut current_poly = self.initial_poly.clone();
        
        // Round loop
        for var_idx in 0..(self.n_vars - 1) {
            let r_i = transcript.squeeze();
            println!("Prover's challenge r_{}: {:?}", var_idx, r_i);
            current_poly = self.partial_evaluate(&current_poly,(0, r_i));
            
            
            // Calculate remaining variables
            let n_vars_remaining = (current_poly.len() as f64).log2() as usize;
            

            // Only split if we have more than one variable left
            if n_vars_remaining > 1 {
                // changed remove_var from var_idx + 1 to 0
                // let (y0, y1) = create_paired_arrays_from_poly(&current_poly, var_idx + 1);
                let (y0, y1) = create_paired_arrays_from_poly(&current_poly, 0);
                
                // Calculate evaluations for next univariate poly
                let mut eval_0 = F::zero();
                let mut eval_1 = F::zero();
                
                for i in 0..y0.len() {
                    eval_0 += y0[i];
                    eval_1 += y1[i];
                }
                
                let next_univariate_evals = vec![eval_0, eval_1];
                transcript.absorb(&to_bytes(next_univariate_evals.clone()));
                
                univariate_polys.push(UnivariatePoly { 
                    evals: next_univariate_evals 
                });
            } else {
                // For the final variable, use the current polynomial values directly
                let next_univariate_evals = current_poly.clone();
                transcript.absorb(&to_bytes(next_univariate_evals.clone()));
                
                univariate_polys.push(UnivariatePoly { 
                    evals: next_univariate_evals 
                });
            }
        }

        let proof = Proof {
            initial_claimed_sum,
            proof: univariate_polys,
        };
        proof
    }
   fn partial_evaluate(&self, poly: &[F], index_value: (usize, F)) -> Vec<F> {
        let pairs = create_paired_arrays_from_poly( poly, index_value.0);
        let vec1 = &pairs.0;
        let vec2 = &pairs.1;
        let mut result_vec = Vec::new();
        
        for index in 0..pairs.0.len() {
            result_vec.push(interpolate(vec1[index], vec2[index], index_value.1));
        }

        result_vec
    }

    fn create_paired_arrays(&self, remove_var: usize) -> (Vec<F>, Vec<F>) {
        let mut vec_zero = Vec::new();
        let mut vec_one = Vec::new();
        let bit_position = self.n_vars - 1 - remove_var;

        for cube_index in 0..self.initial_poly.len() {
            let value = (cube_index >> bit_position) & 1;
            
            if value == 0 {
                vec_zero.push(self.initial_poly[cube_index]);
            } else {
                vec_one.push(self.initial_poly[cube_index]);
            }
        }
        (vec_zero, vec_one)
    }
}


fn create_paired_arrays_from_poly<F: PrimeField>(poly: &[F], remove_var: usize) -> (Vec<F>, Vec<F>) {
    let mut vec_zero = Vec::new();
    let mut vec_one = Vec::new();
    
    // Calculate remaining variables from polynomial size
    let n_vars = (poly.len() as f64).log2() as usize;  // Since poly.len() = 2^n_vars
    
    // Ensure remove_var is in bounds
    if remove_var >= n_vars {
        panic!("remove_var {} is out of bounds for polynomial of size {}", remove_var, poly.len());
    }
    
    let bit_position = n_vars - 1 - remove_var;
    
    for cube_index in 0..poly.len() {
        let value = (cube_index >> bit_position) & 1;
        if value == 0 {
            vec_zero.push(poly[cube_index]);
        } else {
            vec_one.push(poly[cube_index]);
        }
    }
    (vec_zero, vec_one)
}
fn to_bytes<F: PrimeField>(input: Vec<F>) -> Vec<u8> {
    let mut bytes = Vec::new();
        for coeff in &input {
            bytes.extend_from_slice(&coeff.into_bigint().to_bytes_be());
        }
    bytes

}

impl <F:PrimeField>Verifier<F> {
    fn new(n_vars: usize, initial_poly: Vec<F>) -> Self {
        assert_eq!(initial_poly.len(), 1<< n_vars, "Initial polynomial length must be 2^n_vars");
        Self {
            initial_poly,
            n_vars
        }
    }

    fn verify(&self, proof: Proof<F>) -> bool {
        // 1. Initialize transcript
        let hasher = Keccak256Hasher::new();
        let mut transcript: Transcript<Keccak256Hasher, F> = Transcript::init(hasher);
        
        // 2. Absorb the initial polynomial
        let bytes = to_bytes(self.initial_poly.clone());
        transcript.absorb(&bytes);

        // 3. Calculate initial claimed sum
        let init_sum = proof.initial_claimed_sum;

        
        // 4. Absorb the claimed sum
        transcript.absorb(&to_bytes(vec![init_sum]));
        
        // 5. Get univariate polynomial for the first variable from proof
        
        let univariate_evals = &proof.proof[0].evals;  // First univariate polynomial

        // check if evaluating univariate is equal claimed sum
        // let evaluate_initial_poly = interpolate(a, b, x)
        let init_result = univariate_evals[0] + univariate_evals[1];

        assert_eq!(init_sum, init_result);
        
        // 6. Absorb the univariate polynomial
        transcript.absorb(&to_bytes(univariate_evals.clone()));
        
        // Begin squeezing phase

        // store random values 
        let mut rand_vals: Vec<F> = Vec::new();
        for var_idx in 0..(self.n_vars - 1) {
            // squeeze challenge 
            let r_i = transcript.squeeze();
            println!("Verifier's Challenge r_{}: {:?}", var_idx, r_i);
            rand_vals.push(r_i);

            let current_poly = &proof.proof[var_idx].evals;
            let next_poly = &proof.proof[var_idx + 1].evals;

            // evaluate the univariate poly at r_i
            let eval_at_r = interpolate(current_poly[0], current_poly[1], r_i);
            let next_sum = next_poly[0] + next_poly[1];
            
            if eval_at_r != next_sum {
            return false;
        }
            if var_idx < self.n_vars - 1 {
                transcript.absorb(&to_bytes(next_poly.to_vec()));
            }
        // panic!("i got here")
        }

        // Get final random point
        let r_final = transcript.squeeze();
        rand_vals.push(r_final);

        let final_poly = &proof.proof[proof.proof.len()-1].evals;
        let final_ev = interpolate(final_poly[0], final_poly[1], r_final);

        // Check against oracle evaluation
        let oracle_eval = self.evaluate_oracle_at_point(rand_vals);
        assert_eq!(final_ev,oracle_eval);
        true
    }

    fn evaluate_oracle_at_point(&self, points: Vec<F>) -> F {
        let mut current_poly = self.initial_poly.clone();
        
        // For each point, evaluate one variable
        for (idx, point) in points.iter().enumerate() {
            // Calculate remaining variables
            let n_vars_remaining = (current_poly.len() as f64).log2() as usize;
            
            // Only evaluate if we have more than one value
            if current_poly.len() > 1 {
                current_poly = self.partial_evaluate(&current_poly, (0, *point));
            }
        }
        
        current_poly[0]
    }

    fn partial_evaluate(&self, poly: &[F], index_value: (usize, F)) -> Vec<F> {
        let pairs = create_paired_arrays_from_poly(poly, index_value.0);
        let vec1 = &pairs.0;
        let vec2 = &pairs.1;
        let mut result_vec = Vec::new();
        
        for index in 0..pairs.0.len() {
            result_vec.push(interpolate(vec1[index], vec2[index], index_value.1));
        }

        result_vec
    }




    fn create_paired_arrays(&self, remove_var: usize) -> (Vec<F>, Vec<F>) {
        let mut vec_zero = Vec::new();
        let mut vec_one = Vec::new();
        let bit_position = self.n_vars - 1 - remove_var;

        for cube_index in 0..self.initial_poly.len() {
            let value = (cube_index >> bit_position) & 1;
            
            if value == 0 {
                vec_zero.push(self.initial_poly[cube_index]);
            } else {
                vec_one.push(self.initial_poly[cube_index]);
            }
        }
        (vec_zero, vec_one)
    }

}

fn interpolate<F: PrimeField>(y0: F, y1: F, r: F) -> F {
    y0 + r * (y1 - y0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::{One, Zero};
    use ark_bn254::Fr;

    fn get_test_poly() -> Vec<Fr> {
        vec![
            Fr::zero(), // 000
            Fr::zero(), // 001
            Fr::zero(), // 010
            Fr::from(3), // 011
            Fr::zero(), // 100
            Fr::zero(), // 101
            Fr::from(2), // 110
            Fr::from(5), // 111
        ]
    }

    #[test]
    fn test_prover_initialization() {
        let initial_poly = get_test_poly();
        let prover = Prover::new(3, initial_poly.clone());
        assert_eq!(prover.n_vars, 3);
        assert_eq!(prover.initial_poly, initial_poly);
    }

    #[test]
    fn test_create_paired_arrays() {
        let initial_poly = get_test_poly();
        let prover = Prover::new(3, initial_poly);

        // Test splitting on first variable (x₁)
        let (vec0, vec1) = prover.create_paired_arrays(0);
        assert_eq!(vec0, vec![Fr::zero(), Fr::zero(), Fr::zero(), Fr::from(3)]);
        assert_eq!(vec1, vec![Fr::zero(), Fr::zero(), Fr::from(2), Fr::from(5)]);

        // Test splitting on second variable (x₂)
        let (vec0, vec1) = prover.create_paired_arrays(1);
        assert_eq!(vec0, vec![Fr::zero(), Fr::zero(), Fr::zero(), Fr::zero()]);
        assert_eq!(vec1, vec![Fr::zero(), Fr::from(3), Fr::from(2), Fr::from(5)]);

        // Test splitting on third variable (x₃)
        let (vec0, vec1) = prover.create_paired_arrays(2);
        assert_eq!(vec0, vec![Fr::zero(), Fr::zero(), Fr::zero(), Fr::from(2)]);
        assert_eq!(vec1, vec![Fr::zero(), Fr::from(3), Fr::zero(), Fr::from(5)]);
    }

    #[test]
    fn test_partial_evaluate() {
        let initial_poly = get_test_poly();
        let init_clone = initial_poly.clone();
        let prover = Prover::new(3, initial_poly);

        // Evaluate first variable at 1
        let result = prover.partial_evaluate(&init_clone, (0, Fr::one()));
        assert_eq!(result.len(), 4);

        // Expected result after evaluating x₁ at 1
        let expected = vec![
            interpolate(Fr::zero(), Fr::zero(), Fr::one()),
            interpolate(Fr::zero(), Fr::zero(), Fr::one()),
            interpolate(Fr::zero(), Fr::from(2), Fr::one()),
            interpolate(Fr::from(3), Fr::from(5), Fr::one()),
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_complete_protocol() {
        let initial_poly = get_test_poly();
        let prover = Prover::new(3, initial_poly.clone());
        let verifier = Verifier::new(3, initial_poly.clone());

        let proof = prover.prove();
        assert!(verifier.verify(proof));
    }

    #[test]
    fn test_proof_structure() {
        let initial_poly = get_test_poly();
        let prover = Prover::new(3, initial_poly);
        let proof = prover.prove();

        // Initial sum should be 0 + 0 + 0 + 3 + 0 + 0 + 2 + 5 = 10
        let expected_sum = Fr::from(10);
        assert_eq!(proof.initial_claimed_sum, expected_sum);

        // Should have 3 univariate polynomials (one for each variable)
        assert_eq!(proof.proof.len(), 3);

        // Each univariate polynomial should have 2 evaluations
        for poly in &proof.proof {
            assert_eq!(poly.evals.len(), 2);
        }
    }

    #[test]
    fn test_univariate_evaluations() {
        let initial_poly = get_test_poly();
        let prover = Prover::new(3, initial_poly);
        let proof = prover.prove();

        // First univariate polynomial evaluations
        let first_uni = &proof.proof[0].evals;
        assert_eq!(first_uni.len(), 2);
        
        // Verify sum of evaluations
        let sum = first_uni[0] + first_uni[1];
        assert_eq!(sum, Fr::from(10)); // Should sum to total of polynomial
    }

    #[test]
fn test_transcript_consistency() {
    let initial_poly = get_test_poly();
    let prover = Prover::new(3, initial_poly.clone());
    let verifier = Verifier::new(3, initial_poly.clone());

    // Generate first proof
    let proof1 = prover.prove();

    // Generate second proof
    let proof2 = prover.prove();

    // Both proofs should verify
    assert!(verifier.verify(proof1.clone()));
    assert!(verifier.verify(proof2.clone()));

    // Compare proofs
    assert_eq!(proof1.initial_claimed_sum, proof2.initial_claimed_sum);
    for (i, (poly1, poly2)) in proof1.proof.iter().zip(proof2.proof.iter()).enumerate() {
        assert_eq!(poly1.evals, poly2.evals);
    }
}


}

