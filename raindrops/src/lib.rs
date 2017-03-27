use std::borrow::Cow;

pub fn raindrops(number : u32 ) -> &'static str {
    let mut result : String = String::from("");

    if number % 3 == 0 {
        result.push_str("Pling");
    }
    if number % 5 == 0 {
        result.push_str("Plang");
    }
    if number % 7 == 0 {
        result.push_str("Plong");
    }

    if number % 3 != 0 && number % 5 != 0 && number % 7 != 0 {
        result.push_str("1");
    }

    return &*result.as_str();
}
