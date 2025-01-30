use std::vec;
use ark_ff::PrimeField;

fn main(){
    // let xs = vec![1.0, 3.0, 4.0];
    // let ys = vec![45.0, 205.0, 330.0];
    // let result = interpolate(xs, ys);
    // println!("result{:?}", result);
    println!("Hello, world!");
}

#[derive(Debug)]
struct UnivariatePoly<F: PrimeField> {
    coeffs: Vec<F>,
}

impl <F: PrimeField>UnivariatePoly<F> {
    fn evaluate(&self, _x: F) -> F {
        let mut result = F::zero();
        
        for i in 0..self.coeffs.len() {
                result +=  self.coeffs[i] * _x.pow([i as u64]);
            }
        result
    }

    fn degree(&self) -> usize {
        // for i in self.coeffs.len().rev
        return self.coeffs.len() - 1;
    }

}

fn interpolate<F: PrimeField>(xs: Vec<F>, ys: Vec<F>) -> UnivariatePoly<F> {
    if xs.len() != ys.len() {
        panic!("xs and ys must have the same length");
    }

    let mut result_poly: Vec<F> = vec![F::zero(); xs.len()];

    for i in 0..xs.len() {
        let (numerator, denominator) = lagrange_basis(xs[i], &xs);
        println!("numerator {:?}", numerator);
        println!("denominator {}", denominator);

        let scaled_numerator = scalar_multiplication((ys[i] / denominator), numerator);
        println!("scaled_numerator {:?}", scaled_numerator);
        
        // Add the current polynomial to the result
        result_poly = poly_addition(result_poly, scaled_numerator);

        println!("result_poly {:?}", result_poly);
    }

    UnivariatePoly { coeffs: result_poly }
}


fn lagrange_basis<F: PrimeField>(input: F, interpolating_set: &Vec<F>) -> (Vec<F>, F) {
    let mut numerator: Vec<F> = vec![F::one()]; // Start with the polynomial 1
    let mut denominator = F::one(); // Initialize denominator

    for i in 0..interpolating_set.len() {
        if interpolating_set[i] != input {
            // Create the polynomial for the current point
            let current_poly = vec![-interpolating_set[i], F::one()]; // Represents (x - x_i)

            // Multiply the current polynomial with the existing numerator
            numerator = poly_multiplication(numerator , current_poly);
            
            let mut result = F::zero();
            
            for i in 1..numerator.len() {
                result += numerator[i] * input.pow([i as u64]);
            }

            denominator = numerator[0] + result;
        }
    }
    (numerator, denominator)
}

fn poly_multiplication<F: PrimeField>(vec1: Vec<F>, vec2: Vec<F>) -> Vec<F> {
    // let mut vec_result = Vec::new();
    let mut vec_result: Vec<F> = vec![F::zero(); vec1.len() + vec2.len() - 1];
    for i in 0..vec1.len() {
        for j in 0..vec2.len() {
            
                let index = i + j;
                vec_result[index] += ((vec1[i] * vec2[j]) / F::one()) as F;
        }
    }
    vec_result
}

fn poly_addition<F: PrimeField>(vec1: Vec<F>, vec2: Vec<F>) -> Vec<F> {
    let mut vec_result: Vec<F> = Vec::new();
    if vec1.len() > vec2.len() {
        vec_result = vec1;
        for i in 0..vec2.len() {
            vec_result[i] += vec2[i];
           
        }
        
    } else if vec1.len() < vec2.len() {
        vec_result = vec2;
        for i in 0..vec1.len() {
            vec_result[i] += vec1[i];
           
        }

    } else {
        vec_result = vec1;
        for i in 0..vec_result.len() {
            vec_result[i] += vec2[i];
           
        }
    }
    
    vec_result
}

fn scalar_multiplication<F: PrimeField>(scalar: F, vec: Vec<F>) -> Vec<F> {
    let mut vec_result: Vec<F> = Vec::new();
    for i in 0..vec.len() {
        vec_result.push(vec[i] * scalar);
    }
    vec_result
}


#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::Field;
    use ark_bn254::Fr;

    #[test]
    fn test_scalar_multiplication() {
        let vec = vec![Fr::from(1), Fr::from(2), Fr::from(3)];
        let result = scalar_multiplication(Fr::from(2), vec);
        assert_eq!(result, vec![Fr::from(2), Fr::from(4), Fr::from(6)]);
    }

    #[test]
    fn test_poly_multiplication() {
        let vec1 = vec![Fr::from(2), Fr::from(3)];
        let vec2 = vec![Fr::from(4), Fr::from(6)];
        let result = poly_multiplication::<Fr>(vec1, vec2);
        assert_eq!(result, vec![Fr::from(8), Fr::from(24), Fr::from(18)]);
    }

    #[test]
    fn test_poly_addition() {
        let vec1 = vec![Fr::from(2), Fr::from(3)];
        let vec2 = vec![Fr::from(4), Fr::from(6)];
        let result = poly_addition(vec1, vec2);
        assert_eq!(result, vec![Fr::from(6), Fr::from(9)]);
    }

    #[test]
    fn test_lagrange_basis() {
        let xs = vec![Fr::from(1), Fr::from(2), Fr::from(3)];
        let result = lagrange_basis(Fr::from(2), &xs);
        assert_eq!(result.0, vec![Fr::from(3), Fr::from(-4), Fr::from(1)]);
        
    }

}





// let vec1 = vec![0,2, 3];
    // let vec2 = vec![2, 3];
    // let vec3 = vec![2, 3];
    // let vec4 = vec![2, 4, 6];
    // let vec4 = vec![1,2];
    // let vec_result = poly_multiplication(vec1, vec2);
    //  println!("{:?}", vec_result);
    //  let vec_result1 = scalar_multiplication(2, vec3);
    //  println!("{:?}", vec_result1);
    //  let vec_result2 = poly_addition(vec1, vec2);
    //  println!("{:?}", vec_result2);
    //  let vec_result3 = lagrange_basis(1, &vec4);
    //  println!("{:?}", vec_result3);











// fn poly_addition(vec1: Vec<usize>, vec2: Vec<usize>) -> Vec<usize> {
//     let mut vec_result = vec![0; vec1.len().max(vec2.len())]; // Initialize with the maximum length
//     for i in 0..vec_result.len() {
//         let val1 = if i < vec1.len() { vec1[i] } else { 0 }; // Get value from vec1 or 0 if out of bounds
//         let val2 = if i < vec2.len() { vec2[i] } else { 0 }; // Get value from vec2 or 0 if out of bounds
//         vec_result[i] = val1 + val2; // Sum the values
//     }
//     vec_result
// }

 
// fn lagrange_basis(input: f64, interpolating_set: Vec<f64>) -> (Vec<f64>, f64) {
//     let mut numerator = vec![1;interpolating_set.len()];
//     let mut denominator = 0;
//     for i in 0..interpolating_set.len() {
//         if interpolating_set[i] != input && interpolating_set.len() == 2{
//             numerator = vec![-1 * interpolating_set[i], 1];
//         } else if interpolating_set[i] != input {
//             numerator[i] *= interpolating_set[i];
//         }
//     }
//     let mut result = 0;
//     for i in 1..numerator.len()  {
//         // result = result + (input * numerator[i]).pow(i as u32);
//         result +=  numerator[i] * f64::pow(input, i as u32);
//     }
//     denominator = numerator[0] + result;
//     (numerator, denominator)
// }

// fn test_deno(input: Vec<f64>) -> f64 {
//     let mut result = 0;
//     for i in 1..input.len()  {
//         result = result +  input[i] * f64::pow(2, i as u32);
//     }
//     result
// }

// fn test_deno(input: Vec<f64>) -> f64 {
//     let mut result = 0;
//     // for i in 1..input.len() {
//     //     result += input[i] * 2f64.pow(i as u32);
//     // }
//     // result

//     // let mut result = 0;
//     for i in 0..input.len() {
//         result = result + input[i] * 2f64.pow(i as u32);
//     }
//     result
// }