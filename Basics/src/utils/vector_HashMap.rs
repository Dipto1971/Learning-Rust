use std::collections::HashMap;

fn group_values_by_keys (vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut map: HashMap<String, i32> = HashMap::new();
    for (key, value) in vec {
        map.insert(key, value);
    }
    return map;
}

fn main() {
    let vec = vec![
        (String::from("Dipto"), 25),
        (String::from("Rahul"), 24),
        (String::from("Rahim"), 23),
    ];

    let map = group_values_by_keys(vec);
    println!("{:?}", map);
}