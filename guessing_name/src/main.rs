use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("=============== Guess the number! ===============");

    let secret_number = rand::thread_rng().gen_range(0..=100);

    loop {
        println!("Please input your guess");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line :(");

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", input);

        match input.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
        }
    }
}
