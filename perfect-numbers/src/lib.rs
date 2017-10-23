pub fn classify(number: u64) -> Result<Classification, &'static str> {
    if number < 1 {
        return Err("Number must be positive");
    }

    match calculate_aliquot(number) {
        a if a == number => Ok(Classification::Perfect),
        a if a > number => Ok(Classification::Abundant),
        _ => Ok(Classification::Deficient)
    }
}

fn calculate_aliquot(number: u64) -> u64 {
    let mut sum = 0;

    for n in 1..number {
        if number % n == 0 {
            sum += n;
        }
    }

    sum
}

#[derive(Debug)] #[derive(PartialEq)]
pub enum Classification {
    Perfect,
    Abundant,
    Deficient
}
