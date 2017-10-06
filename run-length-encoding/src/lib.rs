pub fn encode(text: &str) -> String {
    let mut encoded = String::new();

    let mut position = 0;
    let mut count = 0;

    for c in text.chars() {
        if encoded.is_empty() {
            encoded.push_str(format!("{}", c).as_str());
            position += 1;

        } else {
            let idx: usize = position - 1;
            if let Some(x) = encoded.get(idx..idx) {
                if x.chars().eq(c) {
                    count += 1;
                } else if let Some(next) = encoded.get((position as usize)..(position as usize)) {
                    let enc = format!("{}{}", count, next);
                    encoded.insert_str((position - 1) as usize, enc.as_str());
                    count = 0;
                    position += enc.len();
                }
            }
        }
    }

    encoded
}

pub fn decode(text: &str) -> &str {
    unimplemented!()
}
