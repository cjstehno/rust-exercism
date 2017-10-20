// return Ok(x) where x is the number of steps required to reach 1
pub fn collatz(mut n: u64) -> Result<u64, &'static str> {
    if n == 0 {
        return Err("invalid input");
    } else {
        let mut steps: u64 = 0;

        while n != 1 {
            if n % 2 == 0 {
                n = n / 2;
            } else {
                n = (n * 3) + 1;
            }
            steps += 1;
        }

        Ok(steps)
    }
}
