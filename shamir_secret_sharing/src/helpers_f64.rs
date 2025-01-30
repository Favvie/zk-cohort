pub fn evaluate(coefficients: &Vec<f64>, point: f64) -> f64 {
    //let poly = [6, 3, 2] => 6 + 3x + 2x^2
    let mut result = 0.0;
    for i in 0..coefficients.len() {
        result = result + coefficients[i] * point.powf(i as f64);
    }

    result
}

pub fn interpolate(xs: Vec<f64>, ys: Vec<f64>) -> Vec<f64> {
    if xs.len() != ys.len() {
        panic!("xs and ys must have the same length");
    }

    let mut result_poly: Vec<f64> = vec![0.0; xs.len()];

    for i in 0..xs.len() {
        let (numerator, denominator) = lagrange_basis(xs[i], &xs);
        println!("numerator {:?}", numerator);
        println!("denominator {}", denominator);

        let scaled_numerator = scalar_multiplication((ys[i] / denominator) as f64, numerator);
        println!("scaled_numerator {:?}", scaled_numerator);
        
        // Add the current polynomial to the result
        result_poly = poly_addition(result_poly, scaled_numerator);

        println!("result_poly {:?}", result_poly);
    }

    result_poly
}


pub fn lagrange_basis(input: f64, interpolating_set: &Vec<f64>) -> (Vec<f64>, f64) {
    let mut numerator: Vec<f64> = vec![1.0]; // Start with the polynomial 1
    let mut denominator = 1.0; // Initialize denominator

    for i in 0..interpolating_set.len() {
        if interpolating_set[i] != input {
            // Create the polynomial for the current point
            let current_poly = vec![-interpolating_set[i], 1.0]; // Represents (x - x_i)

            // Multiply the current polynomial with the existing numerator
            numerator = poly_multiplication(numerator , current_poly);
            
            let mut result = 0.0;
            
            for i in 1..numerator.len() {
                result += numerator[i] * input.powf((i as u32).into());
            }

            denominator = numerator[0] + result;
        }
    }
    (numerator, denominator)
}

pub fn poly_multiplication(vec1: Vec<f64>, vec2: Vec<f64>) -> Vec<f64> {
    // let mut vec_result = Vec::new();
    let mut vec_result: Vec<f64> = vec![0.0; vec1.len() + vec2.len() - 1];
    for i in 0..vec1.len() {
        for j in 0..vec2.len() {
            
                let index = i + j;
                vec_result[index] += ((vec1[i] * vec2[j]) / 1.0) as f64;
            
        }
    }
    vec_result
}

pub fn poly_addition(vec1: Vec<f64>, vec2: Vec<f64>) -> Vec<f64> {
    let mut vec_result: Vec<f64> = Vec::new();
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

pub fn scalar_multiplication(scalar: f64, vec: Vec<f64>) -> Vec<f64> {
    let mut vec_result: Vec<f64> = Vec::new();
    for i in 0..vec.len() {
        vec_result.push((vec[i] / 1.0) as f64 * scalar);
    }
    vec_result
}
