pub fn encrypt(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    let mut clean_text = String::new();

    for c in text.to_lowercase().chars() {
        if c.is_alphabetic() {
            clean_text.push(c);
        }
    }

    let char_count: u64 = clean_text.len() as u64;
    let row_count: u64 = (char_count as f64).sqrt() as u64;
    let col_count = if clean_text.len() as u64 % row_count == 0 { row_count } else { row_count + 1 };

    println!("Counts: chars:{}, rows:{}, cols:{}", char_count, row_count, col_count);

    let mut lines: Vec<String> = vec![];

    for _ in 0..row_count {
        let mut row = String::new();

        for _ in 0..col_count {
            if !clean_text.is_empty() {
                row.push(clean_text.remove(0));
            }
        }

        lines.push(row);
    }

    println!("{:?}", lines);

    let mut output = String::new();

    for c in 0..col_count {
        for r in 0..row_count {
            if let Some(line) = lines.get(r as usize) {
                if let Some(ch) = line.chars().nth(c as usize) {
                    output.push(ch);
                }
            }
        }
    }

    println!("Output: {}", output.trim().to_string() );

    // write out the spaces

    let mut spaced_output = String::new();
    let extra_spaces = i64::abs(row_count as i64 - col_count as i64);

    let mut counter = 0;
    for ch in output.chars(){
        if counter == row_count {
            spaced_output.push(' ');
            counter = 0;
        }

        spaced_output.push(ch);
        counter += 1;

        
    }

    spaced_output.trim().to_string()
}
