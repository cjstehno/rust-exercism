// return Ok(x) where x is the number of steps required to reach 1
pub fn collatz(mut n: u64) -> Result<u64, &'static str> {
    if n == 0 {
        return Err("invalid input");
    } else {
        let mut steps: u64 = 0;

        while n != 1 {
            if n % 2 == 0 {
                // divide n by 2 to get n / 2
                n = n / 2;
            } else {
                // multiply n by 3 and add 1 to get 3n + 1
                n = (n * 3) + 1;
            }
            steps += 1;
        }

        Ok(steps)
    }
}
