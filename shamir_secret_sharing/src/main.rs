use rand::Rng;
use ark_ff::PrimeField;

mod helpers;
use helpers::{evaluate, interpolate}; // Import specific functions

fn main() {
    println!("Hello world");
}

// decide on secret -> 6
// choose a threshold (k) -> 3 (minimum no of shares to reconstruct secret)
// create a polynomial of degree (k -1) => 3-1 = 2
// polynomial => f(x) = a0 + a1x + a2x^2 => 6 + 3x + 2x^2 
//a0 = 6
//a1 = 3
//a2 = 2

// function for constructing the polynomial
// function for generating shares
// function for reconstructing the secret

fn construct_polynomial<F: PrimeField>(secret: F, threshold: isize) -> Vec<F> {
    let  mut rng = rand::thread_rng();
    let mut result = vec![secret];

    for _i in 0..threshold - 1 {
        let random_number: F = F::from(rng.gen_range(0..10));
        result.push(random_number);
    }
    result
}

fn generate_shares<F: PrimeField>(polynomial: &Vec<F>, no_of_shares: isize) -> Vec<(F, F)> {
// generate share by evaluating the polynomial at a random point
        let mut rng = rand::thread_rng();
        let mut shares = vec![(F::zero(),F::zero()); no_of_shares as usize];
        for i in 0..no_of_shares {
            let random_number = F::from(rng.gen_range(1..100));
            let result = evaluate(polynomial, random_number);
            shares[i as usize] = (random_number, result)

        }
        println!("{:?}", shares);
        shares
}

fn reconstruct_secret<F: PrimeField>(threshold: isize, shares: &Vec<(F, F)>) -> F {
    // what does it mean to reconstruct the secret? 
    // you need the threshold, you need shares of the secret of at least of threshold no. 
    // need to check that the length of shares vec is equal or greater than threshold
    // need to interpolate the shares to get a polynomial, the constant is the secret
    if (shares.len() as isize) < threshold {
        panic!("You need a minimum of threshold shares to reconstruct");
    }

    let mut xs = vec![F::zero(); shares.len()];
    let mut ys = vec![F::zero(); shares.len()];

    for i in 0..shares.len() {
        xs[i] = shares[i].0;
        ys[i] = shares[i].1;
    }
    let poly = interpolate(xs, ys);
    poly[0] 
}










#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;

    #[test]
    fn test_construct_polynomial() {
        let secret = Fr::from(6);
        let threshold = 3;

        let result = construct_polynomial(secret, threshold);

        assert_eq!(result[0], secret);
        assert_eq!(result.len(), threshold as usize);
    }

    #[test]
    fn test_evaluate() {
        let poly = vec![Fr::from(6), Fr::from(3), Fr::from(2)];
        let point = Fr::from(2);
        let result = evaluate(&poly, point);
        assert_eq!(result, Fr::from(20));
    }

    #[test]
    fn test_generate_shares() {
        // let poly = vec![6, 3, 2];
        let coeffs = construct_polynomial(Fr::from(6), 3);
        let no_of_shares = 5;
        let result = generate_shares(&coeffs, no_of_shares);
        assert_eq!(result.len(), no_of_shares as usize);

        let unique_shares: std::collections::HashSet<_> = result.iter().collect();
        assert_eq!(unique_shares.len(), result.len());
    }

    #[test]
    fn test_reconstruct_secret() {
        let secret = Fr::from(6);
        let threshold = 3;
        let coeffs = construct_polynomial(secret, threshold);
        println!("coeffs: {:?}", coeffs);

        // Generate shares
        let no_of_shares = 5;
        let shares = generate_shares(&coeffs, no_of_shares);
        println!("shares: {:?}", shares);
        
        // Use only the first 'threshold' shares to reconstruct the secret
        let shares_to_use = &shares[0..threshold as usize];
        println!("shares_to_use: {:?}", shares_to_use);
        let reconstructed_secret = reconstruct_secret(threshold, &shares_to_use.to_vec());
        println!("reconstructed_secret: {:?}", reconstructed_secret);
        
        assert_eq!(reconstructed_secret, secret);
    }

}