pub fn collatz(mut n: u64) -> Option<u64> {
    let mut count: u64 = 0;

    if n == 0 {
        return None;
    }

    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
        count = count + 1;
    }

    Some(count)
}
