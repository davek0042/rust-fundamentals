use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

fn generate_random_array<const N: usize>() -> [i32; N] {
    // Comment out the next line to use random seed instead of fixed seed
    // let mut rng = StdRng::seed_from_u64(442);
    
    // Uncomment the next line to use random seed
    let mut rng = StdRng::from_entropy();
    
    let mut arr = [0; N];
    for i in 0..N {
        arr[i] = rng.gen_range(1..=100);
    }
    arr
}

fn process_numbers(numbers: &[i32]) {
    // Initialize the sum to zero
    let mut sum = 0;

    // Iterate over the numbers, adding each one to the sum
    for number in numbers {
        sum += number;
    }

    // Print the sum
    println!("The sum of the numbers is: {}", sum);

    // If the sum is even, print a message
    if sum % 2 == 0 {
        println!("The sum is even");
    } else {
        println!("The sum is odd");
    }
}


fn main() {
    // Generate an array of N random numbers between 1 and 100
    const NUMBER_OF_ELEMENTS: usize = 25;
    let random_arr = generate_random_array::<NUMBER_OF_ELEMENTS>();
    println!("Random array (length: {}): {:?}", random_arr.len(), random_arr);
    process_numbers(&random_arr);
    
    println!("\n---\n");
    
    // Generate an array of 5 random numbers
    // let random_arr_5 = generate_random_array::<5>();
    // println!("Random array (5): {:?}", random_arr_5);
    // process_numbers(&random_arr_5);
}
