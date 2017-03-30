
pub fn is_valid(card_num: &str) -> bool {
    return is_number_valid(card_num) && luhn_checksum(card_num) == 0
}

fn luhn_checksum(card_num: &str) -> u32 {
    let digits : Vec<u32> = digits_of(card_num);
    let mut even_digits : Vec<u32> = Vec::new();
    let mut odd_digits : Vec<u32> = Vec::new();

    let mut evens = false;
    for d in digits {
        if evens {
            even_digits.push(d);
        } else {
            odd_digits.push(d);
        }

        evens = !evens;
    }

    let mut sum: u32 = odd_digits.iter().sum();

    for d in even_digits {
        sum += sum_digits(2 * d);
    }

    return sum % 10;
}

fn digits_of(number: &str) -> Vec<u32> {
    return number.chars().rev()
        .filter(|c| !c.is_whitespace() && c.is_numeric() )
        .map(|c| c.to_digit(10).unwrap() )
        .collect::<Vec<u32>>();
}

fn sum_digits(number: u32) -> u32 {
    return number.to_string().chars().map(|x| x.to_digit(10).unwrap() ).sum();
}

fn is_number_valid(card_num: &str) -> bool {
    return card_num.trim().len() > 1 && card_num.chars().filter(|x| !x.is_whitespace()).all(|x| x.is_numeric() );
}
