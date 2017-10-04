pub fn find() -> Option<u64> {
    for a in 1..1000 {
        for b in 1..1000 {
            for c in 1..1000 {
                if is_triplet(a, b, c) {
                    return Some(a * b * c);
                }
            }
        }
    }
    return None;
}

fn is_triplet(a: u64, b: u64, c: u64) -> bool {
    a.pow(2) + b.pow(2) == c.pow(2) && a + b + c == 1000
}
