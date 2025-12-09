// There are other conditionals that we can explore in Rust. Like using `if let`

fn main() {
    // let maybe_number: Option<Option<()>> = None;
    let maybe_number = Some(Some(42));
    if let Some(Some(number)) = maybe_number {
        println!("The number is {:?}", number);
    } else if let Some(inner) = maybe_number {
        println!("The inner value is {:?}", inner);
    } else {
        println!("There is no number");
    }
}
