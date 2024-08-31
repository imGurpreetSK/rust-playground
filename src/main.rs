fn main() {
    let person = Person::new("Gurpreet", "Singh");
    println!("{} {}", person.first_name, person.last_name)
}

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }
}
