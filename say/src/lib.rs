const NUMBER_NAMES: [&str; 20] = [
    "", " one", " two", " three", " four", " five", " six", " seven", " eight", " nine", " ten", " eleven",
    " twelve", " thirteen", " fourteen", " fifteen", " sixteen", " seventeen", " eighteen", " nineteen"
];

const TENS_NAMES: [&str; 10] = [
    "", " ten", " twenty", " thirty", " forty", " fifty", " sixty", " seventy", " eighty", " ninety"
];

pub fn encode(number: u64) -> String {
    if number == 0 {
        return String::from("zero");
    }

    let mut words: String = String::new();
    let mut current = number;

    if current > 999_999_999_999_999_999 {
        words.push_str(&format!("{} quintillion ", encode_part(current / 1_000_000_000_000_000_000)));
        current = current % 1_000_000_000_000_000_000;
    }

    if current > 999_999_999_999_999 {
        words.push_str(&format!("{} quadrillion ", encode_part(current / 1_000_000_000_000_000)));
        current = current % 1_000_000_000_000_000;
    }

    if current > 999_999_999_999 {
        words.push_str(&format!("{} trillion ", encode_part(current / 1_000_000_000_000)));
        current = current % 1_000_000_000_000;
    }

    if current > 999_999_999 {
        words.push_str(&format!("{} billion ", encode_part(current / 1_000_000_000)));
        current = current % 1_000_000_000;
    }

    if current > 999_999 {
        words.push_str(&format!("{} million ", encode_part(current / 1_000_000)));
        current = current % 1_000_000;
    }

    if current > 999 {
        words.push_str(&format!("{} thousand ", encode_part(current / 1_000)));
        current = current % 1_000;
    }

    words.push_str(encode_part(current).trim());

    return words.trim().to_string();
}

fn encode_part(number: u64) -> String {
    let mut words: String;

    let mut current = number;

    if current % 100 < 20 {
        words = NUMBER_NAMES[(current % 100) as usize].to_string();
        current /= 100;
    } else {
        words = NUMBER_NAMES[(current % 10) as usize].to_string();
        current /= 10;

        if words.is_empty() {
            words = format!("{}{}", TENS_NAMES[(current % 10) as usize], words.trim());
        } else {
            words = format!("{}-{}", TENS_NAMES[(current % 10) as usize], words.trim());
        }
        current /= 10;
    }

    if current == 0 {
        return words.trim().to_string();
    }

    return format!("{} hundred{}", NUMBER_NAMES[current as usize], words).trim().to_string();
}
