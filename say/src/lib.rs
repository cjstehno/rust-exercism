const NUMBER_NAMES: [&str; 20] = [
    "", " one", " two", " three", " four", " five", " six", " seven", " eight", " nine", " ten", " eleven",
    " twelve", " thirteen", " fourteen", " fifteen", " sixteen", " seventeen", " eighteen", " nineteen"
];

const TENS_NAMES: [&str; 10] = [
    "", " ten", " twenty", " thirty", " forty", " fifty", " sixty", " seventy", " eighty", " ninety"
];

pub fn encode(mut number: u64) -> String {
    if number == 0 {
        return String::from("zero");
    }

    let mut words: String = String::new();

    encode_for(&mut words, &mut number, 1_000_000_000_000_000_000, "quintillion");
    encode_for(&mut words, &mut number, 1_000_000_000_000_000, "quadrillion");
    encode_for(&mut words, &mut number, 1_000_000_000_000, "trillion");
    encode_for(&mut words, &mut number, 1_000_000_000, "billion");
    encode_for(&mut words, &mut number, 1_000_000, "million");
    encode_for(&mut words, &mut number, 1_000, "thousand");

    words.push_str(encode_part(number).trim());

    return words.trim().to_string();
}

fn encode_for(words: &mut String, current: &mut u64, divisor: u64, label: &'static str) {
    if *current > (divisor - 1) {
        words.push_str(&format!("{} {} ", encode_part(*current / divisor), label));
        *current = *current % divisor;
    }
}

fn encode_part(mut number: u64) -> String {
    let mut words: String;

    if number % 100 < 20 {
        words = NUMBER_NAMES[(number % 100) as usize].to_string();
        number /= 100;
    } else {
        words = NUMBER_NAMES[(number % 10) as usize].to_string();
        number /= 10;

        words = format!("{}{}{}", TENS_NAMES[(number % 10) as usize], if words.is_empty() { "" } else { "-" }, words.trim());
        number /= 10;
    }

    if number == 0 {
        return words.trim().to_string();
    }

    return format!("{} hundred{}", NUMBER_NAMES[number as usize], words).trim().to_string();
}
