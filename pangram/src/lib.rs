use std::collections::HashSet;

pub fn is_pangram(sentence: &str) -> bool {
    let mut letter_counts = HashSet::new();

    for ch in sentence.to_uppercase().chars() {
        letter_counts.insert(ch);
    }

    let all_letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();
    return all_letters.iter().all(|letter| letter_counts.contains(letter));
}
