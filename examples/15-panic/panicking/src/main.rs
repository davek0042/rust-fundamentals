fn loop_and_check(numbers: Vec<i32>) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    
    for num in numbers {
        if num < 0 {
            errors.push(format!("Negative number found: {}", num));
        } else {
            println!("Number: {}", num);
        }
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}


fn main() {
    match loop_and_check(vec![1, 2, -3, 4, -5]) {
        Ok(_) => println!("All numbers processed successfully!"),
        Err(errors) => {
            println!("Encountered {} error(s):", errors.len());
            for error in errors {
                println!("  - {}", error);
            }
        }
    }
}
