use std::collections::HashMap;

pub fn word_count( text: &str ) -> HashMap<String, u32> {
    let mut words: HashMap<String, u32> = HashMap::new();

    for word in text.split_whitespace().map(word_mapper).filter(|w| !w.is_empty()) {
        let count = match words.get(&word) {
            Some(&count) => count,
            None         => 0
        };

        words.insert(word, count + 1);
    }

    return words;
}

fn word_mapper( word: &str ) -> String {
    return word.trim_matches(|c: char| !c.is_alphanumeric() ).to_string().to_lowercase();
}
