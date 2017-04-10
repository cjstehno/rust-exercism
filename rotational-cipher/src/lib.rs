
pub fn rotate( text: &str, rot: u16 ) -> String {
    let letters: Vec<char> = "abcdefghijklmnopqrstuvwxyz".to_string().chars().collect::<Vec<char>>();
    let mut swapped: Vec<char> = vec![];

    for ch in text.to_lowercase().chars() {
        swapped.push( match letters.iter().position(|&x| x == ch ) {
            Some(pos) => *letters.get(offset(pos, rot)).unwrap(),
            None      => ' '
        } );
    }

    let output : String = swapped.into_iter().collect();
    return output.trim().to_string();
}

fn offset( pos: usize, rot: u16 ) -> usize {
    let index = pos + rot as usize;
    return if index > 25 { index - 26 } else { index }
}
