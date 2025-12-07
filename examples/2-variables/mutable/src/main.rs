fn main() {
    let mut message = String::from("Name: Alfredo, Height: ");
    message.clear();
    let mut height = 190;
    height = 189;
    message = String::from(r#"Name: David, Height: "#);
    println!("{}{}", message, height);

}
