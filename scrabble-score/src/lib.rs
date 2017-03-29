
pub fn score(word: &str) -> u16 {
    return word.to_uppercase().chars().fold(0, |result, letter| result + letter_score(letter) );
}

fn letter_score(letter: char) -> u16 {
    return match letter {
        'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
        'D' | 'G'                                                 => 2,
        'B' | 'C' | 'M' | 'P'                                     => 3,
        'F' | 'H' | 'V' | 'W' | 'Y'                               => 4,
        'K'                                                       => 5,
        'J' | 'X'                                                 => 8,
        'Q' | 'Z'                                                 => 10,
        _                                                         => 0
    }
}
