
pub fn square_of_sum(number: u32) -> u32 {
    let mut sum = 0;
    for n in 1..number+1 {
        sum += n;
    }
    return sum.pow(2);
}

pub fn sum_of_squares(number: u32) -> u32 {
    let mut sum = 0;
    for n in 1..number+1 {
        sum += n.pow(2);
    }
    return sum;
}

pub fn difference(number: u32) -> u32 {
    return square_of_sum(number) - sum_of_squares(number);
}
