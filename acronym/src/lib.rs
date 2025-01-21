pub fn abbreviate(phrase: &str) -> String {
    if phrase.is_empty() {
        return String::new();
    }

    let mut result: Vec<char> = vec![];
    let mut i = 0;
    while i < phrase.len() - 1 {
        if i == 0 {
            result.push(phrase.chars().nth(0).unwrap());
        } else if phrase.chars().nth(i).unwrap() == ' ' {
            while !phrase.chars().nth(i).unwrap().is_ascii_alphabetic() {
                i += 1;
            }
            result.push(phrase.chars().nth(i).unwrap());
        } else if phrase.chars().nth(i).unwrap() == '-' {
            while !phrase.chars().nth(i).unwrap().is_ascii_alphabetic() {
                i += 1;
            }
            result.push(phrase.chars().nth(i).unwrap());
        } else if phrase.chars().nth(i).unwrap().is_ascii_lowercase() && phrase.chars().nth(i + 1).unwrap().is_uppercase() {
            result.push(phrase.chars().nth(i + 1).unwrap());
        }
        i += 1
    }

    result.iter().collect::<String>().to_uppercase()
}
