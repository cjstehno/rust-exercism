
pub fn encode( text: &str ) -> String {
    let letters: Vec<u8> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string().into_bytes();

    return String::from_utf8( text.chars().map(|c| match letters.iter().position(|&x| x == c as u8) {
        Some(idx) => letters.get_mut(25 - idx).unwrap(),
        None      => &mut " ".to_string().into_bytes()[0]
    } ).collect::<Vec<& mut u8>>() ).unwrap();
}

pub fn decode( text: &str ) -> String {
    unimplemented!()
}

// def letters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'.toCharArray() as List
//
// 'The quick brown fox jumps over the lazy dog'.toCharArray().collect { c->
//     if( c == ' '){ return ' ' }
//
//     int letter = letters.indexOf(c.toUpperCase())
//     letters[25-letter]
// }
