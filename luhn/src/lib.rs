
pub fn is_valid(card_num: &str) -> bool {
    if card_num.len() < 2 {
        return false;
    }

    let mut numbers : Vec<u32> = card_num.chars().rev().filter(|c| !c.is_whitespace()).map(|c| c.to_digit(10).unwrap() ).collect::<Vec<u32>>();

    let mut doubler = false;
    for n in 0..numbers.len() {
        if doubler {
            let double : u32 = numbers[n] * 2;
            println!("Double: {}" , double);
            numbers[n] = match double {
                0 => 0,
                _ if double > 9 => sum_digits(double),
                _ => double - 9
            }
        }

        numbers[n] = sum_digits(numbers[n]);

        doubler = !doubler;
    }

    let sum : u32 = numbers.iter().sum();
    println!("Sum: {}", sum);

    let check: u32 = sum % 10;
    println!("Mod: {}", check);

    let result: bool = sum.to_string().chars().last().unwrap().to_digit(10).unwrap() == check;
    return result;
}

fn sum_digits(number: u32) -> u32 {
    return number.to_string().chars().map(|x| x.to_digit(10).unwrap() ).sum();
}
