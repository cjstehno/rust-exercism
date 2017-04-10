
pub fn encode( text: &str ) -> String {
    let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".to_string().chars().collect::<Vec<char>>();
    let mut swapped: Vec<char> = vec![];

    let mut char_count: u16 = 0;

    for ch in text.to_lowercase().chars().filter(|&c| c != ' ') {
        if ch.is_digit(10) {
            swapped.push(ch);
        } else if ch.is_alphabetic() {
            swapped.push( match letters.iter().position(|&x| x == ch) {
                Some(pos) => *letters.get(25 - pos).unwrap(),
                None      => ' '
            } );
        } else {
            continue;
        }

        char_count = char_count + 1;

        if char_count == 5 {
            char_count = 0;
            swapped.push(' ');
        }
    }

    return swapped.into_iter().collect();
}

pub fn decode( text: &str ) -> String {
    unimplemented!()
}
