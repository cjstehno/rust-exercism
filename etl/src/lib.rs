use std::collections::BTreeMap;

pub fn transform( input : &BTreeMap<i32, Vec<String>> ) -> BTreeMap<String, i32> {
    let mut output : BTreeMap<String,i32> = BTreeMap::new();

    for (id, words) in input.iter() {
        for word in words.iter() {
            output.insert( word.to_lowercase(), *id );
        }
    }

    output
}
