use std::collections::BTreeSet;

pub fn sum_of_multiples(number: u32, multiples: &Vec<u32>) -> u32 {
    let mut numbers = BTreeSet::new();

    for n in 1..number {
        for m in multiples {
            if n % m == 0 {
                numbers.insert(n);
            }
        }
    }

    numbers.iter().fold(0, |sum,n| sum + n)
}
