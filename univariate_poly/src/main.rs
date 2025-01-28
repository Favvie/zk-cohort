use std::vec;

fn main(){
    println!("Hello, world!");
    }

#[derive(Debug)]
struct UnivariatePoly {
    coeffs: Vec<isize>,
}

impl UnivariatePoly {
    fn evaluate(&self, _x: isize) -> isize {
        let mut result = 0;
        
        for i in 0..self.coeffs.len() {
                result +=   self.coeffs[i] * isize::pow(_x, i as u32);
            }
        result
    }

    fn degree(&self) -> usize {
        // for i in self.coeffs.len().rev
        return self.coeffs.len() - 1;
    }

}

fn interpolate(xs: Vec<isize>, ys: Vec<isize>) -> UnivariatePoly {
    if xs.len() != ys.len() {
        panic!("xs and ys must have the same length");
    }

    let mut result_poly: Vec<isize> = vec![0; xs.len()];

    for i in 0..xs.len() {
        let (numerator, denominator) = lagrange_basis(xs[i], &xs);
        println!("numerator {:?}", numerator);
        println!("denominator {}", denominator);

        let scaled_numerator = scalar_multiplication(ys[i] / denominator, numerator);
        println!("scaled_numerator {:?}", scaled_numerator);
        
        // Add the current polynomial to the result
        result_poly = poly_addition(result_poly, scaled_numerator);

        println!("result_poly {:?}", result_poly);
    }

    UnivariatePoly { coeffs: result_poly }
}


fn lagrange_basis(input: isize, interpolating_set: &Vec<isize>) -> (Vec<isize>, isize) {
    let mut numerator: Vec<isize> = vec![1]; // Start with the polynomial 1
    let mut denominator = 1; // Initialize denominator

    for i in 0..interpolating_set.len() {
        if interpolating_set[i] != input {
            // Create the polynomial for the current point
            let current_poly = vec![-interpolating_set[i], 1]; // Represents (x - x_i)

            // Multiply the current polynomial with the existing numerator
            numerator = poly_multiplication(numerator , current_poly);
            
            let mut result = 0;
            
            for i in 1..numerator.len() {
                result +=   numerator[i] * isize::pow(input, i as u32);
            }

            denominator = numerator[0] + result;

            

        }
    }
    (numerator, denominator)
}

fn poly_multiplication(vec1: Vec<isize>, vec2: Vec<isize>) -> Vec<isize> {
    // let mut vec_result = Vec::new();
    let mut vec_result = vec![0; vec1.len() + vec2.len() - 1];
    for i in 0..vec1.len() {
        for j in 0..vec2.len() {
            
                let index = i + j;
                vec_result[index] += vec1[i] * vec2[j];
            
        }
    }
    vec_result
}

fn poly_addition(vec1: Vec<isize>, vec2: Vec<isize>) -> Vec<isize> {
    let mut vec_result = Vec::new();
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

fn scalar_multiplication(scalar: isize, vec: Vec<isize>) -> Vec<isize> {
    let mut vec_result = Vec::new();
    for i in 0..vec.len() {
        vec_result.push(vec[i] * scalar);
    }
    vec_result
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scalar_multiplication() {
        let vec = vec![1, 2, 3];
        let result = scalar_multiplication(2, vec);
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_poly_multiplication() {
        let vec1 = vec![2, 3];
        let vec2 = vec![4, 6];
        let result = poly_multiplication(vec1, vec2);
        assert_eq!(result, vec![8, 24, 18]);
    }

    #[test]
    fn test_poly_addition() {
        let vec1 = vec![2, 3];
        let vec2 = vec![4, 6];
        let result = poly_addition(vec1, vec2);
        assert_eq!(result, vec![6, 9]);
    }

    #[test]
    fn test_lagrange_basis() {
        let xs = vec![1, 2, 3];
        let result = lagrange_basis(2, &xs);
        assert_eq!(result.0, vec![3, -4, 1]);
        
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

 
// fn lagrange_basis(input: isize, interpolating_set: Vec<isize>) -> (Vec<isize>, isize) {
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
//         result +=  numerator[i] * isize::pow(input, i as u32);
//     }
//     denominator = numerator[0] + result;
//     (numerator, denominator)
// }

// fn test_deno(input: Vec<isize>) -> isize {
//     let mut result = 0;
//     for i in 1..input.len()  {
//         result = result +  input[i] * isize::pow(2, i as u32);
//     }
//     result
// }

// fn test_deno(input: Vec<isize>) -> isize {
//     let mut result = 0;
//     // for i in 1..input.len() {
//     //     result += input[i] * 2isize.pow(i as u32);
//     // }
//     // result

//     // let mut result = 0;
//     for i in 0..input.len() {
//         result = result + input[i] * 2isize.pow(i as u32);
//     }
//     result
// }