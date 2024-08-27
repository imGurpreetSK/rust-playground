fn main() {
    // Arrays are always homogenous.
    let array: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    let slice1: &[i32] = &array[0..2];
    let slice2: &[i32] = &array[2..7];

    println!("array is {:?}", array);
    println!("slice1 is {:?}", slice1); // Slices are *always* borrowed. A copy is never made.
    println!("slice2 is {:?}", slice2) // Very intimate relationship with arrays.
}
