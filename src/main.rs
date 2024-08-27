fn main() {
    // Rust does *not* have try..catch.
    // It does have _Optional_ values.

    let array: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];

    let first: Option<&i32> = array.get(0);
    let last: Option<&i32> = array.get(6);
    let invalid: Option<&i32> = array.get(11);

    println!("array is {:?}", array);
    println!("First: {}, {}", first.is_some(), *first.unwrap());
    println!("Last: {}, {}", last.is_some(), *last.unwrap());
    println!("Invalid: {}, {}", invalid.is_some(), *invalid.unwrap_or(&0)); // invalid's value is None; panics!
}
