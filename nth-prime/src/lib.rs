pub fn nth(n: u16) -> Result<u32, ()> {
    if n == 0 {
        return Err(());
    } else {
        let mut candidate: u32 = 2;
        let mut count: u16 = 0;

        while count < n {
            if is_prime(candidate) {
                count += 1;
            }
            candidate += 1;
        }

        return Ok(candidate - 1);
    }
}

fn is_prime(number: u32) -> bool {
    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }

    return true;
}
