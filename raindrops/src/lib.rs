
/// Converts a number to a string, the contents of which depend on the number's factors.
///
/// * If the number has 3 as a factor, output 'Pling'.
/// * If the number has 5 as a factor, output 'Plang'.
/// * If the number has 7 as a factor, output 'Plong'.
/// * If the number does not have 3, 5, or 7 as a factor, just pass the number's digits straight through.
///
pub fn raindrops(number : u32) -> &'static str {
    if number % 3 == 0 {
        return "Pling";
    }
    if number % 5 == 0 {
        return "Plang";
    }
    if number % 7 == 0 {
        return "Plong";
    }

    // let s : &str = number.to_string().to_owned().as_str();
    // return s;
}
