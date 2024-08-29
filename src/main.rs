fn dump(s: &str) {
    println!("str '{}'", s);
}

// Read: https://stevedonovan.github.io/rust-gentle-intro/1-basics.html#strings
fn main() {
    let text = "hello dolly";  // the string slice
    let s = text.to_string();  // it's now an allocated string
    let _string = String::new();

    dump(text);
    dump(&s);

    let text = "the red fox and the lazy dog";
    let output: String = text.chars()
        .filter(|x| { !x.is_whitespace() })
        .collect();
    println!("{output}");

    for i in text.chars() {
        if !i.is_whitespace() {
            print!("{i}");
        }
    }
}
