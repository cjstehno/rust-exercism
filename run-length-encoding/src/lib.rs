pub fn encode(text: &str) -> String {
    let mut encoded = Vec::<String>::new();

    let mut count = 1;
    let mut current: Option<char> = None;

    for c in text.chars() {
        if let None = current {
            current = Some(c);
        } else if let Some(cur) = current {
            if cur == c {
                count += 1;
            } else {
                push_encoded(&mut encoded, count, cur);
                current = Some(c);
                count = 1;
            }
        }
    }

    // pick up the last char
    if let Some(cur) = current {
        push_encoded(&mut encoded, count, cur);
    }

    encoded.join("")
}

fn push_encoded(encoded: &mut Vec<String>, count: u16, current: char) {
    if count > 1 {
        encoded.push(format!("{}{}", count, current));
    } else {
        encoded.push(format!("{}", current));
    }
}

pub fn decode(text: &str) -> String {
    let mut decoded = Vec::<String>::new();
    let mut digits = Vec::<String>::new();

    for c in text.chars() {
        if c.is_numeric() {
            digits.push(c.to_string());
        } else {
            let count = digits.join("").parse().unwrap_or(1);
            decoded.push(c.to_string().repeat(count));
            digits.clear();
        }
    }

    decoded.join("")
}
