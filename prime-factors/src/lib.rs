pub fn factors(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    let mut num = n;
    let mut range = (2..);
    while num > 1 {
        let i = range.next().unwrap();
        while num % i == 0 {
            factors.push(i);
            num /= i;
        }
    }
    
    factors
}
