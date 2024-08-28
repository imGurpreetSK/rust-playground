fn main() {
    let mut v1 = vec![1, 2, 3, 4];
    v1.pop();
    v1.push(11);

    let mut v2 = vec![9, 8, 7];
    v2.extend(5..7);

    println!("{:?} {:?}", v1, v2)
}
