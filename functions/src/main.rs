fn main() {
    println!("Hello, world!");
    another_function(7);
    println!("{}", yet_another_function());
}

fn yet_another_function() -> i32 {
    5
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
