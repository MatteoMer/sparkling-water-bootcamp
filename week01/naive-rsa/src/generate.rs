use crate::utils::modinv;
extern crate rand;

use base64::{Engine as _, engine::general_purpose};
use std::ops::Mul;
use num_bigint::BigInt;
use crate::utils::is_prime;
use rand::RngCore;


fn generate_random_large_prime(num_bytes: u32) -> BigInt {
    let mut random_bigint: BigInt;

    loop {
        let mut rng = rand::thread_rng();
        let mut bytes = vec![0u8; num_bytes.try_into().unwrap()];
        rng.fill_bytes(&mut bytes);

        random_bigint = BigInt::from_bytes_be(num_bigint::Sign::Plus, &bytes);
        if random_bigint.clone() % BigInt::from(2) == BigInt::from(0) {continue}
        if is_prime(&random_bigint) {return random_bigint}
    }

}

pub fn generate(e: u64, num_bytes: u32) {

    let e = BigInt::from(e);

    //p, q = 2 primes, p_1, q_1 = p-1, q-1
    
    let p = generate_random_large_prime(num_bytes);
    let p_1: BigInt = p.clone() - 1;
    let q = generate_random_large_prime(num_bytes);
    let q_1: BigInt = q.clone() - 1;

    let n = p.mul(q);
    let phi_n = p_1.mul(q_1);

    let d = modinv(e.clone(), phi_n);

    let public_key = format!("{};{}",e, n);
    let private_key = format!("{};{}",d, n);

    println!("Your public key is: {}", general_purpose::STANDARD_NO_PAD.encode(public_key));
    println!("Your private key is: {}", general_purpose::STANDARD_NO_PAD.encode(private_key));

}