pub fn primes_up_to(max: u16) -> Vec<u16> {
    let mut candidates: Vec<bool> = vec![true; (max + 1) as usize];
    candidates[0] = false;
    candidates[1] = false;

    let mut factor = 2;
    while factor * factor <= max {
        if candidates[factor as usize] {
            let mut j = factor;
            while factor * j <= max {
                candidates[(factor * j) as usize] = false;
                j += 1;
            }
        }

        factor += 1;
    }

    candidates.iter().enumerate().filter(|x| *x.1).map(|x| x.0 as u16).collect()
}
