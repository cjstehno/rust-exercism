
pub fn hamming_distance(first: &str, second: &str) -> Result<usize,()> {
    if first.len() == second.len() {
        let count: usize = first.chars().zip(second.chars()).filter(|cs| cs.0 != cs.1 ).count();
        return Ok(count);

    } else {
        return Err(());
    }
}
