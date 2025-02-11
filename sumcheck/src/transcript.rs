use std::marker::PhantomData;
use sha3::{Keccak256, Digest};
use ark_ff::PrimeField;

pub struct Transcript<K: HashTrait, F: PrimeField> {
    _field: PhantomData<F>,
    hash_function: K,
}

impl<K: HashTrait, F: PrimeField> Transcript<K, F> {
    pub fn init(hash_function: K) -> Self {
        Self {
            _field: PhantomData,
            hash_function,
        }
    }

    pub fn absorb(&mut self, data: &[u8]) {
        self.hash_function.append(data);
    }

    pub fn squeeze(&self) -> F {
        let hash_output = self.hash_function.generate_hash();
        // F::from_be_bytes_mod_order(&hash_output)
        F::from(2)
    }
}

pub trait HashTrait {
    fn append(&mut self, data: &[u8]);
    fn generate_hash(&self) -> Vec<u8>;
}

// Custom struct to wrap Keccak256
pub struct Keccak256Hasher {
    hasher: Keccak256,
    current_state: Vec<u8>
}

impl Keccak256Hasher {
    pub fn new() -> Self {
        Self {
            hasher: Keccak256::new(),
            current_state: Vec::new(),
        }
    }
}

impl HashTrait for Keccak256Hasher {
    fn append(&mut self, data: &[u8]) {
        self.hasher.update(data);

        self.current_state = self.hasher.clone().finalize().to_vec();
    }

    fn generate_hash(&self) -> Vec<u8> {
        // self.hasher.clone().finalize().to_vec()
        self.current_state.clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ark_bn254::Fq;
    use ark_ff::{BigInteger, PrimeField};

    #[test]
    fn test_hash() {
        // Use the Keccak256Hasher wrapper
        let hasher = Keccak256Hasher::new();
        let mut transcript = Transcript::<Keccak256Hasher, Fq>::init(hasher);

        // Absorb data into the transcript
        transcript.absorb(Fq::from(7).into_bigint().to_bytes_be().as_slice());
        transcript.absorb("girl".as_bytes());

        // Squeeze challenges
        let challenge = transcript.squeeze();
        let challenge1 = transcript.squeeze();

        dbg!(challenge);
        dbg!(challenge1);
    }
}


















































// use std::marker::PhantomData;
// use sha3::{Keccak256, Digest};
// use ark_ff::PrimeField;

// pub struct Transcript <K: HashTrait, F: PrimeField> {
//     _field: PhantomData<F>,
//     hash_function: K
// }

// impl <K: HashTrait, F: PrimeField> Transcript<K, F> {
//     pub fn init(hash_function: K) -> Self {
//         Self {_field: PhantomData, hash_function}
//     }

//     fn absorb(&mut self, data: &[u8]) {
//         self.hash_function.append(data);
//     }

//     fn squeeze(&self) -> F {
//         let hash_output = self.hash_function.generate_hash();
//         F::from_be_bytes_mod_order(&hash_output)
//     }
// }


// trait HashTrait {
//     fn append(&mut self, data: &[u8]);
//     fn generate_hash(&self) -> Vec<u8>;
// }


// impl HashTrait for Keccak256 {
//     fn append(&mut self, data: &[u8]) {
//         self.update(data)
//     }

//     fn generate_hash(&self) -> Vec<u8> {
//         self.clone().finalize().to_vec()
//     }
// }



// #[cfg(test)]
// mod test {

//     use super::Transcript;
//     use ark_bn254::Fq;
//     use ark_ff::{BigInteger, PrimeField};
//     use super::Keccak256;
//     use sha3::Digest;


//     #[test]
//     fn test_hash() {

//         let mut transcript = Transcript::<Keccak256, Fq>::init(Keccak256::new());

//         transcript.absorb(Fq::from(7).into_bigint().to_bytes_be().as_slice());
//         transcript.absorb("girl".as_bytes());

//         let challenge = transcript.squeeze();
//         let challenge1 = transcript.squeeze();

//         dbg!(challenge);
//         dbg!(challenge1);
//     }
// }