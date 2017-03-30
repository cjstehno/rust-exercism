use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let found_letters = sentence.to_uppercase().chars().collect::<HashSet<char>>();

    let all_letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();
    return all_letters.iter().all(|letter| found_letters.contains(letter));
}
