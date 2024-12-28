pub fn nth(n: usize) -> u32 {
    let mut primes: Vec<u32> = vec![];
    (2..)
        .filter(|n| {
            if primes.iter().any(|i| { n % i == 0 }) {
                false
            } else {
                primes.push(*n);
                true
            }
        })
        .nth(n)
        .unwrap()
}
