pub fn encode(number: u64) -> String {
    match number {
        _ if number < 10 => single_digits(number),
        _ if number < 20 => teens(number),
        // TODO: more in here
        20 => String::from("twenty"),
        21 => String::from("twenty-one"),
        22 => String::from("twenty-two"),
        _ if number < 1000 => hundreds(number),
        _ => panic!("Unsupported number value")
    }
}

fn single_digits(number: u64) -> String {
    match number {
        0 => String::from("zero"),
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        _ => panic!("invalid number")
    }
}

fn teens(number: u64) -> String {
    match number {
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        14 => String::from("fourteen"),
        15 => String::from("fifteen"),
        16 => String::from("sixteen"),
        17 => String::from("seventeen"),
        18 => String::from("eighteen"),
        19 => String::from("nineteen"),
        _ => panic!("invalid number")
    }
}

fn hundreds(number: u64) -> String {
    match number {

    }
}

/*

    0 - 9 : single_digits
    10 - 19 : teens
    20 - 29 : twenties
    ... thirties, fourties, fifties, ...
    100 ...
    1000

    encode ( n / 100)
*/