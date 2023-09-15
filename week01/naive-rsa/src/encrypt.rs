use num_bigint::BigUint;

pub fn encrypt(message: &str, e: u64, n: &str) {
    let e = BigUint::from(e);
    let n = BigUint::parse_bytes(n.as_bytes(), 10).expect("Invalid number");
    println!("n: {}", n);


    // Convert the message bytes directly to a BigUint
    let m = BigUint::from_bytes_be(message.as_bytes());
    println!("m: {}", m);

    let bytes = m.to_bytes_be();

    let recovered_message = String::from_utf8(bytes).expect("Invalid UTF-8 sequence");
    println!("{}", recovered_message);

    // Encryption
    let encrypted_message = m.modpow(&e, &n);
    // println!("Your encrypted message is: {}", encrypted_message);
    println!("c: {}", encrypted_message);
}