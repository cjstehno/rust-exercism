use std::cmp::max;

pub fn lsp(digit_string: &str, count: usize) -> Result<u32,()> {
    if count > digit_string.len() || digit_string.chars().any(|c| !c.is_numeric()) {
        return Err(());
    }

    if count == 0 || digit_string.len() == 0 {
        return Ok(1);
    }

    let mut max_product : u32 = 0;
    for series in digit_string.chars().map(to_digit).collect::<Vec<u32>>().windows(count) {
        max_product = max( max_product, series.iter().product() );
    }

    return Ok(max_product);
}

fn to_digit(ch : char) -> u32 {
    return ch.to_digit(10).unwrap();
}
