
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
