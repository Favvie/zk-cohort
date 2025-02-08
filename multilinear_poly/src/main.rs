use std::result;

use ark_ff::PrimeField;

fn main() {
    println!("Hello, world!");
}


#[derive(Debug, Clone)]
struct MultilinearPoly<F: PrimeField> {
    evals: Vec<F>,
    n_vars: usize
}

impl <F: PrimeField>MultilinearPoly<F> {
    fn new( n_vars: usize, evaluations: Vec<F>) -> Self {

        Self{
            evals: evaluations,
            n_vars 
        }
    }

    fn evaluate(&self, points: Vec<F>) -> Vec<F>{
        // let mut vecx = Vec::new();
        let mut poly = self.clone();
        for i in 0..points.len() {
            poly = poly.partial_evaluate((0, points[i]));
            // println!("index, vecx {} {:?}", i, vecx)
        }
        poly.evals
    }

    
    fn partial_evaluate(&self, index_value: (usize, F)) -> MultilinearPoly<F> {
        let pairs = self.create_paired_arrays( index_value.0);
        let vec1 = &pairs.0;
        let vec2 = &pairs.1;
        let mut result_vec = Vec::new();
        
        for index in 0..pairs.0.len() {
            result_vec.push(interpolate(vec1[index], vec2[index], index_value.1));
        }
        let new_multi: MultilinearPoly<F> =  MultilinearPoly::new(self.n_vars - 1, result_vec);
        new_multi
    }

    fn create_paired_arrays(&self, remove_var: usize) -> (Vec<F>, Vec<F>) {
        let mut vec_zero = Vec::new();
        let mut vec_one = Vec::new();
  
        println!("n_vars:{}, remove_var:{}", self.n_vars, remove_var);
        let bit_position = self.n_vars - 1 - remove_var;

        for cube_index in 0..self.evals.len() {
            let value = (cube_index >> bit_position) & 1;
            
            if value == 0 {
                vec_zero.push(self.evals[cube_index]);
            } else {
                vec_one.push(self.evals[cube_index]);
            }
        }
        // dbg!(vec_zero.clone(), vec_one.clone());

        (vec_zero, vec_one)
    }
}

fn interpolate<F: PrimeField>(y0: F, y1: F, a: F) -> F {
    y0 + a * (y1 - y0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;
    
    #[test]
    fn test_create_paired_arrays() {
        let evals = vec![
            Fr::from(0),
            Fr::from(0),
            Fr::from(0),
            Fr::from(3),
            Fr::from(0),
            Fr::from(0),
            Fr::from(2),
            Fr::from(5),
        ];
        let poly = MultilinearPoly { n_vars: 3, evals };
        
        let (vec_zero, vec_one) = poly.create_paired_arrays(1);
        
        assert_eq!(vec_zero, vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(0)]);
        assert_eq!(vec_one, vec![Fr::from(0), Fr::from(3), Fr::from(2), Fr::from(5)]);
    }

    #[test]
    fn test_partial_evaluate() {
        // Create a polynomial with the specified evaluations
        let evals = vec![
            Fr::from(0), // 000
            Fr::from(0), // 001
            Fr::from(0), // 010
            Fr::from(3), // 011
            Fr::from(0), // 100
            Fr::from(0), // 101
            Fr::from(2), // 110
            Fr::from(5), // 111
        ];
        let poly = MultilinearPoly::new( 3, evals);
        
        // Evaluate using the partial_evaluate function
        let result = poly.partial_evaluate((0, Fr::from(3)));
        
        let expected = vec![Fr::from(0), Fr::from(0), Fr::from(6), Fr::from(9)];
        println!("{:?}",result);
        
        // assert_eq!(result, expected);
    }

    #[test]
    fn test_evaluate() {
        let eval = vec![Fr::from(0), Fr::from(0), Fr::from(0), Fr::from(3), Fr::from(0), Fr::from(0), Fr::from(2), Fr::from(5)];
        let multipoly = MultilinearPoly::new(3, eval);
        let points = vec![Fr::from(3),Fr::from(2),Fr::from(1)];
        let result = multipoly.evaluate(points);
        assert_eq!(result[0], Fr::from(18));
    }
}

