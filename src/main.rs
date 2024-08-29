fn main() {
    // Don't get comfortable with panic.
    let arg = std::env::args().nth(1).expect("no arguments supplied");
    let i = arg.parse::<i32>().expect("not an integer!");
    println!("{i}")
}
