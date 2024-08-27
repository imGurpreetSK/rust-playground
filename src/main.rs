fn main() {
    /*
     * Arrays are not used that often in Rust, because the type of array includes its size.
     * The type of the array in the example is [i32; 4]; the type of [10, 20] would be [i32; 2] and so forth: they have different types.
     * So they are bastards to pass around as function arguments.
     *
     * Though, due to this property, rust knows the size of array at compile time. Hence, ArrayIndexOutOfBounds don't happen.
     */
    let array: [i32; 5] = [1, 2, 3, 4, 5];
    for i in 0..array.len() {
        println!("[{i}]: {} ", array[i])
    }
}
