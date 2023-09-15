use num_bigint::BigInt;
use num_integer::Integer;
use num_traits::identities::{Zero, One};

pub fn modinv(a0: BigInt, m0: BigInt) -> BigInt {
    if m0 == BigInt::one() {return BigInt::one()}

    let (mut a, mut m, mut x0, mut inv) = (a0, m0.clone(), BigInt::zero(), BigInt::one());
    
    while a > BigInt::one() {
        let (div, rem) = a.div_rem(&m);
        inv -= div * &x0;
        a = rem;
        std::mem::swap(&mut a, &mut m);
        std::mem::swap(&mut x0, &mut inv)
    }
    if inv < BigInt::zero() { inv += m0 }
    inv
}