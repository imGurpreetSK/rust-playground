pub fn private_key(p: u64) -> u64 {
    (p - 1) / 2
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    calculate(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    calculate(b_pub, a, p)
}

fn calculate(base: u64, exp: u64, modulus: u64) -> u64 {
    // (base ^ exp) mod modulus
    
    if modulus == 1 {
        return 0;
    }

    let mut result = 1;
    let mut base = (base % modulus) as u128;
    let mut exp = exp;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus as u128;
        }
        exp = exp >> 1;
        base = (base * base) % modulus as u128;
    }
    
    result as u64
}
