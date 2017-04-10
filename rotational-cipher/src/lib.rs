
pub fn rotate( text: &str, rot: u16 ) -> String {
    let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".to_string().chars().collect::<Vec<char>>();
    let mut swapped: Vec<char> = vec![];

    for ch in text.chars() {
        if ch.is_digit(10) {
            swapped.push(ch);

        } else {
            let letter: char = match letters.iter().position(|&x| x == ch.to_lowercase().next().unwrap() ) {
                Some(pos) => *letters.get(offset(pos, rot)).unwrap(),
                None      => ch
            };
            swapped.push(if ch.is_uppercase() { letter.to_uppercase().next().unwrap() } else { letter });
        }
    }

    let output : String = swapped.into_iter().collect();
    return output.trim().to_string();
}

fn offset( pos: usize, rot: u16 ) -> usize {
    let index = pos + rot as usize;
    return if index > 25 { index - 26 } else { index }
}
