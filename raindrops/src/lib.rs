
/// Converts a number to a string, the contents of which depend on the number's factors.
///
/// * If the number has 3 as a factor, output 'Pling'.
/// * If the number has 5 as a factor, output 'Plang'.
/// * If the number has 7 as a factor, output 'Plong'.
/// * If the number does not have 3, 5, or 7 as a factor, just pass the number's digits straight through.
///
pub fn raindrops(number : u32 ) -> String {
    let mut drops : String = String::from("");

    if number % 3 == 0 {
        drops.push_str("Pling");
    }
    if number % 5 == 0 {
        drops.push_str("Plang");
    }
    if number % 7 == 0 {
        drops.push_str("Plong");
    }

    if number % 3 != 0 && number % 5 != 0 && number % 7 != 0 {
        drops.push_str(&number.to_string());
    }

    return drops;
}
