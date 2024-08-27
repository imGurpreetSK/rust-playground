fn main() {
    println!("Square is {}", square(2.0));
    println!("Clamped: {}", clamp(3.0, 5.0, 7.0));
}

fn clamp(x: f64, lower: f64, upper: f64) -> f64 {
    if x < lower {
        lower
    } else if x > upper {
        upper
    } else {
        x
    }
}

fn square(x: f64) -> f64 {
   x * x
}
