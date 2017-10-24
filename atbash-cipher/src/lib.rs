use std::ascii::AsciiExt;

const LETTERS: &'static str = "abcdefghijklmnopqrstuvwxyz";

pub fn encode( text: &str ) -> String {
    codex(text, true)
}

pub fn decode( text: &str ) -> String {
    codex(text, false)
}

fn codex( text: &str, chunked: bool ) -> String {
    let letters: Vec<char> = LETTERS.to_string().chars().collect::<Vec<char>>();
    let mut swapped: Vec<char> = vec![];

    let mut char_count: u16  = 0;

    for ch in text.to_lowercase().chars().filter(|&c| c != ' ' && c.is_ascii()) {
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

        char_count += 1;

        if chunked && char_count % 5 == 0 {
            swapped.push(' ');
        }
    }

    let output : String = swapped.into_iter().collect();
    output.trim().to_string()
}
