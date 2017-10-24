use std::collections::HashSet;

const LETTERS: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn is_pangram(sentence: &str) -> bool {
    let found_letters = sentence.to_uppercase().chars().collect::<HashSet<char>>();
    
    return LETTERS.chars().all(|letter| found_letters.contains(&letter));
}
