fn main() {
    let mut height = 190;
    height = height - 20;
    let result = if height < 180 {
        "Tall"
    } else if height > 170 {
        "Average"
    } else {
        "Short"
    };

    println!("Result: {}", result);

    // why is this line generating an unused variable warning?
    
    let health = if height < 180 {"good"} else {"unknown"};
    println!("Health: {}", health);
    
    // shadowing to a different type
    // works because we're changing the type of health with "let"
    let health = if height < 180 {true} else {false};

}
