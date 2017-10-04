pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }

    match s {
        1 => 1,
        _ => (2..s+1).fold(1, |grains, _| grains * 2)
    }
}

pub fn total() -> u64 {
    (1..65).fold(0, |sum, s| sum + square(s) )
}
