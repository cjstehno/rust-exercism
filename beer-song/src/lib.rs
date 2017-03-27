use std::iter::range_step;

pub fn verse(number : u16 ) -> String {
    return match number {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        n @ _ => fill_bottles(n)
    }
}

#[allow(unused_variables)]
pub fn sing(start: u16, end: u16) -> String {
    let mut song : String = String::from("foo");

    for v in range_step(start, end, -1) {
        song.push_str("bar");
        song.push_str(&verse(v).to_string());
    }

    return song;
}

fn fill_bottles(number: u16) -> String {
    return format!(
        "{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {next} bottle{plural} of beer on the wall.\n",
        n = number, next = number-1, plural = if number - 1 > 1 { "s" } else { "" }
    );
}
