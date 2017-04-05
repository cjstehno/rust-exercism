
pub fn encode( text: &str ) -> String {
    let letters = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();
    let codex = letters.iter().enumerate();

    return text.chars().map({ |c|
        match codex.find(|x| x == c) {
            Some(idx) => letters.index(25-idx),
            None      => " "
        }
    });
}

// def letters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'.toCharArray() as List
//
// 'The quick brown fox jumps over the lazy dog'.toCharArray().collect { c->
//     if( c == ' '){ return ' ' }
//
//     int letter = letters.indexOf(c.toUpperCase())
//     letters[25-letter]
// }
