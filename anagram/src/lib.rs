use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();
    let mut sorted_word = sort(word);
    sorted_word.sort_unstable();
    possible_anagrams.iter().for_each(|item| {
        let mut sorted_item = sort(item);
        sorted_item.sort_unstable();
        if item.to_lowercase() != word.to_lowercase() && sorted_word == sorted_item {
            set.insert(*item);
        }
    });

    set
}

fn sort(word: &str) -> Vec<char> {
    word.to_lowercase().chars().collect::<Vec<_>>()
}
