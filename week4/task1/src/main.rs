//closures
use num_bigint::BigInt;
use num_traits::{One, Zero};

fn main() {
    println!("Welcome to the factorial calculator!");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let input: i32 = input.trim().parse().expect("Please type a number!");

    if input < 0 {
        println!("Factorial is not defined for negative numbers.");
        return;
    }

    
    let factorial = |n:i32|
    {
        let mut result = BigInt::one();
        for i in 2..=n {
            result *= i;
        }
        result
    };

    let result = factorial(input);
    println!("The factorial of {} is {}", input, result);
}


// fn factorial(n: i32) -> BigInt {
//     let mut result = BigInt::one();

//     for i in 2..=n {
//         result *= i;
//     }

//     result
// }

