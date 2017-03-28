
pub fn hamming_distance(first: &String, second: &String) -> Result<usize,()> {
    let count: usize = first.chars().zip(second.chars()).filter(|cs| cs.0 != cs.1 ).count();

    if count > 0 {
        return Ok(count);
    } else {
        return Err(());
    }
}
