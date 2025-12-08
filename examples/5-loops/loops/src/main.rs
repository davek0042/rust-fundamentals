// using the loop keyword is useful to avoid having to define the condition upfront
// or if the condition is met in the middle of the loop
// It is also useful when you want to loop without knowing exactly when to stop

fn main() {
    let mut x = 1;
    
    // Using loop
    loop {
        println!("loop: x is {}", x);
        x += 1;
        if x > 5 {
            break;
        }
    }
    
    // Using while
    x = 1; // reset x
    while x <= 5 {
        println!("while: x is {}", x);
        x += 1;
    }
    
    // Using for
    for x in 1..=5 {
        println!("for: x is {}", x);
    }
}
