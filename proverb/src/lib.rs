pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }

    let mut verses: Vec<String> = vec![];
    for i in 1..list.len() {
        verses.push(format!(
            "For want of a {} the {} was lost.",
            list[i - 1],
            list[i]
        ));
    }
    verses.push(format!("And all for the want of a {}.", list[0]));

    verses.join("\n")
}
