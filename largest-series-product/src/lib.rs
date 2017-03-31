

pub fn lsp(digit_string: &str, count: usize) -> Result<u32,()> {
    if count > digit_string.len() || digit_string.chars().any(|c| !c.is_numeric()) {
        return Err(());
    }

    if count == 0 || digit_string.len() == 0 {
        return Ok(1);
    }

    let mut products : Vec<u32> = vec![];

    let digits = digit_string.chars().map(to_digit).collect::<Vec<u32>>();

    for series in digits.windows(count) {
        let prod : u32 = series.iter().product();
        println!("Product {:?}: {}", series, prod);
        products.push( prod );
    }

    return Ok(*products.iter().max().unwrap());
}

fn to_digit(ch : char) -> u32 {
    return ch.to_digit(10).unwrap();
}
