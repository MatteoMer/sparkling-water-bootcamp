use num_bigint::BigUint;

pub fn decrypt(d: &str, c: &str, n: &str){
    let d = BigUint::parse_bytes(d.as_bytes(), 10).expect("Invalid number");
    let c = BigUint::parse_bytes(c.as_bytes(), 10).expect("Invalid number");
    let n = BigUint::parse_bytes(n.as_bytes(), 10).expect("Invalid number");

    println!("c: {}", c);
    println!("d: {}", d);
    println!("n: {}", n);

    let m = c.modpow(&d, &n);
    println!("m: {}", m);

    let bytes= m.to_bytes_be();

    let recovered_message = String::from_utf8(bytes).expect("Invalid UTF-8 sequence");

    println!("Your message is: {}", recovered_message);
}