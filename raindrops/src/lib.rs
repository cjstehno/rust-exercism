
/// Converts a number to a string, the contents of which depend on the number's factors.
///
/// * If the number has 3 as a factor, output 'Pling'.
/// * If the number has 5 as a factor, output 'Plang'.
/// * If the number has 7 as a factor, output 'Plong'.
/// * If the number does not have 3, 5, or 7 as a factor, just pass the number's digits straight through.
///
pub fn raindrops(number : u32 ) -> String {
    let mut drops = String::new();

    let factor3 = number % 3 == 0;
    let factor5 = number % 5 == 0;
    let factor7 = number % 7 == 0;

    if factor3 {
        drops.push_str("Pling");
    }
    if factor5 {
        drops.push_str("Plang");
    }
    if factor7 {
        drops.push_str("Plong");
    }

    if !factor3 && !factor5 && !factor7 {
        drops.push_str(&number.to_string());
    }

    return drops;
}
