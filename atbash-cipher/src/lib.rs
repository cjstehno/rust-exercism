
pub fn encode( text: &str ) -> String {
    let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".to_string().chars().collect::<Vec<char>>();
    let mut swapped: Vec<char> = vec![];

    for ch in text.to_lowercase().chars() {
        if ch == ' ' { continue }

        let letter: char = match letters.iter().position(|&x| x == ch) {
            Some(pos) => *letters.get(25 - pos).unwrap(),
            None      => ' '
        };

        swapped.push(letter);
    }

    return swapped.into_iter().collect();
}

pub fn decode( text: &str ) -> String {
    unimplemented!()
}
