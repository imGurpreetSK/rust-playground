fn main() {
    let x = 3;
    println!("First: {}", x);
    println!("Second: {}", &x);
    println!("Third: {}", *&x);
}
