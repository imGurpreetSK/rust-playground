use std::collections::HashMap;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let names = ["Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry"];
    let mut plants = HashMap::with_capacity(4);
    plants.insert('G', "grass");
    plants.insert('C', "clover");
    plants.insert('R', "radishes");
    plants.insert('V', "violets");

    let start_position = names.iter().position(|&name| name == student).unwrap() * 2;
    diagram
        .lines()
        .flat_map(|line| {
            line[start_position..=start_position + 1]
                .chars()
                .map(|char| { *plants.get(&char).unwrap() })
        })
        .collect()
}
