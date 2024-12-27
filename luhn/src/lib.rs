/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .filter(|element| { !element.is_whitespace() })
        .rev()
        .try_fold((0, 0), |(index, sum), item| {
            item.to_digit(10)
                .map(|x| { if index % 2 == 1 { x * 2 } else { x } })
                .map(|x| { if x > 9 { x - 9 } else { x } })
                .map(|x| { (index + 1, sum + x) })
        })
        .map_or(false, |(index, sum)| { index > 1 && sum % 10 == 0 })
}
