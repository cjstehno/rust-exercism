pub fn encode(text: &str) -> String {
    let mut encoded = String::new();

    let mut count = 1;
    let mut current: Option<char> = None;

    for c in text.chars() {
        if let None = current {
            current = Some(c);
        } else if let Some(cur) = current {
            if cur == c {
                count += 1;
            } else {
                if count > 1 {
                    encoded.push_str(&format!("{}{}", count, cur));
                } else {
                    encoded.push_str(&format!("{}", cur));
                }

                current = Some(c);
                count = 1;
            }
        }
    }

    // TODO: some refactoring
    if let Some(cur) = current {
        if count > 1 {
            encoded.push_str(&format!("{}{}", count, cur));
        } else {
            encoded.push_str(&format!("{}", cur));
        }
    }

    encoded
}

pub fn decode(text: &str) -> String {
    let mut decoded = Vec::<String>::new();
    let mut digits = Vec::<String>::new();

    for c in text.chars() {
        if c.is_numeric(){
            digits.push(c.to_string());

        } else {
            let count = digits.join("").parse().unwrap_or(1);
            decoded.push(c.to_string().repeat(count));
            digits.clear();
        }
    }

    decoded.join("")
}
