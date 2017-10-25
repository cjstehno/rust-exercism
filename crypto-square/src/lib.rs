pub fn encrypt(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    let clean_text = normalize_input(text);
    println!("Clean-text: {:?}", clean_text);

    let counts: (u64, u64) = calculate_counts(&clean_text);

    let lines: Vec<String> = text_to_rows(clean_text, counts.1 as usize);
    println!("Lines: {:?}", lines);

    let mut output = String::new();

    for c in 0..counts.1 {
        for r in 0..counts.0 {
            if let Some(line) = lines.get(r as usize) {
                if let Some(ch) = line.chars().nth(c as usize) {
                    output.push(ch);
                }
            }
        }
    }

    println!("Output: {}", output.trim().to_string());

    let mut spaced_output = String::new();
    let extra_spaces = i64::abs(counts.0 as i64 - counts.1 as i64);
    println!("Extra-spaces: {}", extra_spaces);

    let mut counter = 0;
    for ch in output.chars() {
        if counter == counts.0 {
            spaced_output.push(' ');
            counter = 0;
        }

        spaced_output.push(ch);
        counter += 1;
    }

    spaced_output.trim().to_string()
}

fn normalize_input(text: &str) -> String {
    let mut clean_text = String::new();

    for c in text.to_lowercase().chars() {
        if c.is_alphabetic() {
            clean_text.push(c);
        }
    }

    clean_text
}

fn calculate_counts(text: &String) -> (u64, u64){
    let char_count: u64 = text.len() as u64;
    let row_count: u64 = (char_count as f64).sqrt() as u64;
    let col_count = if text.len() as u64 % row_count == 0 { row_count } else { row_count + 1 };

    println!("Counts: chars:{}, rows:{}, cols:{}", char_count, row_count, col_count);

    (row_count, col_count)
}

fn text_to_rows(text: String, col_count: usize) -> Vec<String> {
    let mut lines: Vec<String> = vec![];

    let mut starting = 0;
    while starting <= text.len() {
        let mut ending = starting + col_count;
        if ending > text.len(){
            ending = text.len();
        }

        if let Some(txt) = text.get(starting..ending) {
            lines.push(txt.to_string());
        }

        starting += col_count;
    }

    lines
}