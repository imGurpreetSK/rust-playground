pub fn series(digits: &str, len: usize) -> Vec<String> {
    if len > digits.len() { return vec![]; }

    digits.chars()
        .collect::<Vec<char>>()
        .windows(len)
        .map(|w| w.iter().collect::<String>())
        .collect()
}
