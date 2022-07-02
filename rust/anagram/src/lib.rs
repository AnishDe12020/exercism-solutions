use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let to_lowercase = |c: char| c.to_lowercase().to_string();

    let mut word = word.chars().map(to_lowercase).collect::<Vec<_>>();
    let word_unsorted = word.clone();
    word.sort_unstable();

    possible_anagrams
        .iter()
        .filter(|&possible_anagram| {
            let mut possible_anagram = possible_anagram
                .chars()
                .map(to_lowercase)
                .collect::<Vec<_>>();
            if possible_anagram == word_unsorted {
                return false;
            }
            possible_anagram.sort_unstable();
            possible_anagram == word
        })
        .cloned()
        .collect()
}
