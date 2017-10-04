
pub fn square_of_sum(number: u32) -> u32 {
    ((1..number+1).fold(0, |sum, x| sum + x)).pow(2)
}

pub fn sum_of_squares(number: u32) -> u32 {
    (1..number+1).fold(0, |sum, x| sum + x.pow(2))
}

pub fn difference(number: u32) -> u32 {
    square_of_sum(number) - sum_of_squares(number)
}
