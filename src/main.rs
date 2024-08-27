fn main() {
    let array: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    println!("Sum is {}", sum(&array)) // Use '&' to pass slice to function --> Borrowing.
}

fn sum(slice: &[i32]) -> i32 { // Notice slice's type.
    let mut sum = 0;
    for i in 0..slice.len() {
        sum += slice[i];
    }
    return sum
}
