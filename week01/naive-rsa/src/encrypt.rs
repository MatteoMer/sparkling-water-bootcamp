use num_bigint::BigUint;
use base64::{Engine as _, engine::general_purpose};


pub fn encrypt(message: &str, public_key: &str) {

    let public_key = general_purpose::STANDARD_NO_PAD.decode(public_key).unwrap();

    let public_key_str = std::str::from_utf8(&public_key).expect("Invalid UTF-8 sequence");

    let mut keys = public_key_str.split(';');
    let e = keys.next().unwrap_or_default();
    let n = keys.next().unwrap_or_default();

    let e = BigUint::parse_bytes(e.as_bytes(), 10).expect("Invalid number");
    let n = BigUint::parse_bytes(n.as_bytes(), 10).expect("Invalid number");


    // Convert the message bytes directly to a BigUint
    let m = BigUint::from_bytes_be(message.as_bytes());

    // Encryption
    let encrypted_message = m.modpow(&e, &n);
    println!("Your encrypted message is: {}", encrypted_message);
}