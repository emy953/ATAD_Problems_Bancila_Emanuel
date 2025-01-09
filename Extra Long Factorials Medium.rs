use std::io::{self, BufRead};


/*
 * Complete the 'extraLongFactorials' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn extraLongFactorials(n: i32) {
    // Vector to store the result, starting with 1 (the factorial of 0 or 1 is 1)
    let mut result = vec![1u32];
    
    // Loop through numbers 2 to n to calculate the factorial
    for i in 2..=n {
        let mut carry = 0;
        
        // Multiply each digit of result by i, and handle carry
        for j in 0..result.len() {
            let product = result[j] * (i as u32) + carry;
            result[j] = product % 10;  // Store the current digit
            carry = product / 10;      // Carry the rest
        }
        
        // If carry is still left, add it to the result
        while carry > 0 {
            result.push(carry % 10);
            carry /= 10;
        }
    }

    // Print the result in reverse order (since we stored the digits in reverse)
    for digit in result.iter().rev() {
        print!("{}", digit);
    }
    println!(); // Move to the next line
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    extraLongFactorials(n);
}
