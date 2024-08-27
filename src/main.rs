fn main() {
    // Arrays are always homogenous.
    let array1: [i32; 7] = [1, 2, 3, 4, 5, 6, 7];
    let array2: [[i32; 3]; 2] = [[1, 2, 3], [3, 4, 5]];
    let array3: [[&str; 1]; 2] = [["One"], ["Two"]];
    println!("array1 is {:?}", array1); // _Debug_ printing.
    println!("array2 is {:?}", array2); // _Debug_ printing.
    println!("array3 is {:?}", array3) // _Debug_ printing.
}
