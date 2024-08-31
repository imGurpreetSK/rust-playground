fn main() {
    // https://stevedonovan.github.io/rust-gentle-intro/2-structs-enums-lifetimes.html
    let s1 = "Hello dolly".to_string();
    let mut rs1 = &s1;
    {
        let tmp = "hello world".to_string();
        rs1 = &tmp;
    }
    println!("{}", rs1)
}
