fn main() {
    // the for loop using a range. Note you can use also `(1..10)` or `(1..=10)`
    // for i in 1..=10 {
    //     println!("i = {}", i);
    // }

    // for i in (1..=5).rev() {
    //     println!("{}", i);
    // }
    
    // let numbers = vec![1, 2, 3, 4, 5];
    // for n in numbers {
    //     print!("{}, ", n);
    // }
    // print!("\n");

    for i in (0..=40).step_by(2) {
        println!("{},", i);
    }

    // for i in (5..=10).rev() {
    //     println!("{},", i);
    // }

}
