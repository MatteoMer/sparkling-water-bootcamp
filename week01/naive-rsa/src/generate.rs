
use std::ops::Mul;
use num_bigint::BigInt;

use crate::utils::modinv;


pub fn generate(e: u64) {
    let e = BigInt::from(e);

    //p, q = 2 primes, p_1, q_1 = p-1, q-1
    let p = BigInt::parse_bytes(b"84852413240602577954214777008087281843214016299350315794614634297287697200010764533593325786107542326132589753401598693679754757", 10).expect("Invalid number");
    let p_1: BigInt = p.clone() - 1;
    let q = BigInt::parse_bytes(b"50373217868900107291273550384286753462298309990020270133008801440870943881781196521322141014725275347548581897920583302199962017", 10).expect("Invalid number");
    let q_1: BigInt = q.clone() - 1;

    let n = p.mul(q);
    println!("n: {}", n);
    let phi_n = p_1.mul(q_1);

    let d = modinv(e, phi_n);
    println!("d: {}", d);

//     // Convert the message bytes directly to a BigUint
//     let m = BigUint::from_bytes_be(message.as_bytes());

//     // Encryption
//     let encrypted_message = m.modpow(&e, &n);
//     println!("Your encrypted message is: {}", encrypted_message);
}