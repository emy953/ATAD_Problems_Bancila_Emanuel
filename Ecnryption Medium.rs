use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'encryption' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn encryption(s: &str) -> String {
    s.replace(" ", "");
    
    // Get the length of the string
    let length = s.len() as f64;
    
    // Calculate the square root of the length
    let sqrt_value = length.sqrt();
    
    // Calculate the floor and ceiling of the square root
    let floor_value = sqrt_value.floor() as usize;
    let ceiling_value = sqrt_value.ceil() as usize;
    
    let mut result = String::new();  // To store the final result string
    
     // Split the string into rows, each with `x` characters
    let rows: Vec<Vec<char>> = s.chars()
        .collect::<Vec<char>>()
        .chunks(ceiling_value)
        .map(|chunk| chunk.to_vec())
        .collect();


    for col in 0..ceiling_value {
        // Collect the characters for this column
        for row in &rows {
            if col < row.len() {
                result.push(row[col]);  // Append the column element to the result
            }
        }
        result.push(' '); // Add space after each column's characters
    }

    result.trim_end().to_string()  // Remove the trailing space and return the result
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = encryption(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
