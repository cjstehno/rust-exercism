
pub fn hamming_distance(first: &str, second: &str) -> Result<usize,()> {
    return match first.len() == second.len(){
        true => Ok(first.chars().zip(second.chars()).filter(|cs| cs.0 != cs.1 ).count()),
        false => Err(())
    }
}
