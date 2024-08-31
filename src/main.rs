fn main() {
    let person = Person::new("Gurpreet", "Singh");
    println!("{}", person.full_name());

    println!("{:?}", person);

    let mut new = person.copy();
    new.set_first_name("Swaran");

    let t = new.to_tuple();
    println!("{} {}", t.0, t.1);
    // println!("{} {}", new.first_name, new.last_name) // Value has moved; error.
}

/*
    - no self argument: you can associate functions with structs, like the new "constructor".
    - &self argument: can use the values of the struct, but not change them
    - &mut self argument: can modify the values
    - self argument: will consume the value, which will move.
*/

#[derive(Debug)]
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

    fn copy(&self) -> Person {
        Self::new(&self.first_name, &self.last_name)
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_first_name(&mut self, name: &str) {
        self.first_name = name.to_string()
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}
