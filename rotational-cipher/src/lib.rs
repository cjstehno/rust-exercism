const LETTERS : &'static str = "abcdefghijklmnopqrstuvwxyz";

pub fn rotate( text: &str, rot: u16 ) -> String {
    let letters: Vec<char> = LETTERS.to_string().chars().collect::<Vec<char>>();
    let mut swapped: Vec<char> = vec![];

    for ch in text.chars() {
        if ch.is_digit(10) {
            swapped.push(ch);

        } else {
            let letter: char = match letters.iter().position(|&x| x == ch.to_lowercase().next().unwrap() ) {
                Some(pos) => *letters.get(offset(pos, rot)).unwrap(),
                None      => ch
            };
            swapped.push(keep_case(ch.is_uppercase(), letter));
        }
    }

    let output : String = swapped.into_iter().collect();
    output.trim().to_string()
}

fn keep_case( is_upper: bool, letter: char ) -> char {
    if is_upper { letter.to_uppercase().next().unwrap() } else { letter }
}

fn offset( pos: usize, rot: u16 ) -> usize {
    let index = pos + rot as usize;
    if index > 25 { index - 26 } else { index }
}
