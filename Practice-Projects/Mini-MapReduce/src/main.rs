mod map_reduce_functions;

struct KeyValue {
    key: String,
    value: String,
}
fn main() {
    map_reduce_functions::word_count::hello();
    println!("Hello, world!");
    sequential_map_reduce()
}

fn map(filename: String, contents: String) -> KeyValue {
    KeyValue { key: filename, value: contents }
}

fn reduce(key: String, values: &[String]) -> String {
    format!("{}{:?}", key, values)
}

fn sequential_map_reduce() {
    println!("Hello, world! sequential_map_reduce");
    let _ = map("hello".to_string(), "world".to_string());
    let _ = reduce("hello".to_string(), &vec!["".to_string()]);
}
