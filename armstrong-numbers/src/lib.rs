pub fn is_armstrong_number(num: u32) -> bool {
    let string = num.to_string();
    let power = string.len() as u32;
    string.chars()
        .rev()
        .map(|x| { x.to_digit(10).unwrap() })
        .map(|x| { x.pow(power) })
        .sum::<u32>() == num
}
