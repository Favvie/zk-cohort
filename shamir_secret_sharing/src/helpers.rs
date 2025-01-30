use ark_ff::PrimeField;

pub fn evaluate<F: PrimeField>(coefficients: &Vec<F>, point: F) -> F {
    //let poly = [6, 3, 2] => 6 + 3x + 2x^2
    let mut result = F::zero();
    for i in 0..coefficients.len() {
        result = result + coefficients[i] * point.pow([i as u64]);
    }

    result
}

pub fn interpolate<F: PrimeField>(xs: Vec<F>, ys: Vec<F>) -> Vec<F> {
    if xs.len() != ys.len() {
        panic!("xs and ys must have the same length");
    }

    let mut result_poly: Vec<F> = vec![F::zero(); xs.len()];

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

    result_poly 
}


pub fn lagrange_basis<F: PrimeField>(input: F, interpolating_set: &Vec<F>) -> (Vec<F>, F) {
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

pub fn poly_multiplication<F: PrimeField>(vec1: Vec<F>, vec2: Vec<F>) -> Vec<F> {
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

pub fn poly_addition<F: PrimeField>(vec1: Vec<F>, vec2: Vec<F>) -> Vec<F> {
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

pub fn scalar_multiplication<F: PrimeField>(scalar: F, vec: Vec<F>) -> Vec<F> {
    let mut vec_result: Vec<F> = Vec::new();
    for i in 0..vec.len() {
        vec_result.push(vec[i] * scalar);
    }
    vec_result
}
