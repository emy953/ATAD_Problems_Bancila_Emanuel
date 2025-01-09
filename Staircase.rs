use std::io::{self, BufRead};

/*
 * Complete the 'staircase' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn staircase(n: i32) {
    for i in 1..=n {
        // Print (n - i) spaces followed by i '#' characters
        let spaces = (n - i) as usize;
        let hashes = i as usize;
        println!("{}{}", " ".repeat(spaces), "#".repeat(hashes));
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    staircase(n);
}
