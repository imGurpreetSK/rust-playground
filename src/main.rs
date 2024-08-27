fn main() {
    println!("Factorial is {}", factorial(5));
}

fn factorial(n: u64) -> u64 {
   if n == 0 {
       1
   } else {
       n * factorial(n - 1)
   }
}
