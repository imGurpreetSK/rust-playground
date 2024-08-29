fn main() {
    // https://stevedonovan.github.io/rust-gentle-intro/1-basics.html#matching
    let multilingual = "Hi! ¡Hola! привет!";

    match multilingual.find("п") {
        None => { println!("Couldn't find the greeting; Hello!") }
        Some(index) => {
            let hi = &multilingual[index..];
            println!("Russian hi: {}", hi)
        }
    }

    // If only interested in one of the possible results.
    if let Some (index) = multilingual.find("п") {
        println!("Russian hi: {}", &multilingual[index..])
    }

    let n = 8;
    println!(
        "{}",
        match n {
            0..=3 => "small",
            4..=6 => "medium",
            _ => "large"
        }
    )
}
