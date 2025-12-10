// us2e core::num;


fn sum(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result
}

fn average(numbers: &[i32]) -> i32 {
    let mut result = 0;
    for number in numbers {
        result += number;
    }
    result / numbers.len() as i32
}


fn main() {
    use std::io::{self, Write};
    
    print!("Enter numbers separated by spaces:\t");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    
    if numbers.is_empty() {
        println!("No valid numbers entered.");
        return;
    }
    
    let result = sum(&numbers);
    let average_result = average(&numbers);
    println!("The sum is {}", result);
    println!("The average is {}", average_result);
}
