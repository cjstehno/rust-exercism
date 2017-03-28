
pub fn verse(number : u16 ) -> String {
    return match number {
        0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
        n @ _ => fill_bottles(n)
    }
}

pub fn sing(start: u16, end: u16) -> String {
    let mut song = Vec::new();

    for n in (end..start + 1).rev() {
        song.push(verse(n));
    }

    return song.join("\n");
}

fn fill_bottles(number: u16) -> String {
    return format!(
        "{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {next} bottle{plural} of beer on the wall.\n",
        n = number, next = number-1, plural = if number - 1 > 1 { "s" } else { "" }
    );
}
