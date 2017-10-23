pub fn classify(number: u64) -> Result<Classification, &'static str> {
    if number < 1 {
        return Err("Number must be positive");
    }

    Ok(match calculate_aliquot(number) {
        a if a == number => Classification::Perfect,
        a if a > number => Classification::Abundant,
        _ => Classification::Deficient
    })
}

fn calculate_aliquot(number: u64) -> u64 {
    (1..number).filter(|x| number % x == 0 ).fold(0, |acc, x| acc + x)
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Classification {
    Perfect,
    Abundant,
    Deficient
}
