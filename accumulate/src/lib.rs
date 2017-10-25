
pub fn map_function(input: Vec<i32>, fx: &Fn(i32) -> i32) -> Vec<i32> {
    let mut output: Vec<i32> = vec![];

    for i in input {
        output.push(fx(i));
    }

    output
}

pub fn map_closure<F>(input: Vec<i32>, closure: F ) -> Vec<i32> where F:Fn(i32) -> i32{
    map_function(input, &closure)
}