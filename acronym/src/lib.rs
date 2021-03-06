

pub fn abbreviate( name: &str ) -> String {
    let mut abbr : Vec<char> = Vec::new();

    for word in name.replace('-', " ").split_whitespace() {
        let mut word_chars = word.chars();
        abbr.push(word_chars.next().unwrap().to_uppercase().next().unwrap());

        if !word.chars().all(|c| c.is_uppercase() || !c.is_alphabetic()) {
            for cap in word_chars.filter(|c| c.is_uppercase() ) {
                abbr.push(cap);
            }
        }
    }

    abbr.into_iter().collect()
}
