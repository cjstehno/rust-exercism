pub fn factors(number: u64) -> Vec<u64> {
    let mut prime_factors = vec![];

    let mut n: u64 = 1;
    let mut current: u64 = number;

    while n <= number {
        if current % n == 0 {
            if is_prime(n) {
                prime_factors.push(n);
                current = current / n;
                n = 0;
            }
        }

        n += 1;
    }

    prime_factors
}

fn is_prime(number: u64) -> bool {
    if number == 1 { return false; }

    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }

    return true;
}