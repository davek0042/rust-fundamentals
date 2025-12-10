
fn split_string(s: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    
    /*
    This is the AI fix.  It addresses the potential out-of-bounds
    access by using unwrap_or to provide a default value of an empty
    string when the index is out of bounds.
     */
    // let result = parts.get(field);
    // result.unwrap_or(&"").to_string()

    /* 
    this is the class solution 
    */
    let result = parts.get(field);
    result.expect("Field index out of bounds").to_string()
}

fn main() {
    let chunk = split_string("hello,world".to_string(), ',', 1);
    println!("Split string: {}", chunk);
}
