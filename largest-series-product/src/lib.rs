

pub fn lsp(digit_string: &str, count: usize) -> Result<u32,()> {
    let mut products : Vec<u32> = vec![];

    let mut digits = digit_string.chars().peekable();

    while digits.peek().is_some() {
        let series : Vec<u32> = digits.by_ref().take(count).map(to_digit).collect::<Vec<u32>>();
        let prod : u32 = series.iter().product();
        println!("Product {:?}: {}", series, prod);
        products.push( prod );
    }

    return Ok(*products.iter().max().unwrap());
}

fn to_digit(ch : char) -> u32 {
    return ch.to_digit(10).unwrap();
}
