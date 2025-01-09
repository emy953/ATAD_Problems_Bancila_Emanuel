use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    // Extract the period (AM/PM) from the string
    let period = &s[8..10];  // AM/PM is the last 2 characters
    let hour_str = &s[0..2]; // First two characters are the hour
    let minute_and_second = &s[2..8]; // The rest is minute and second

    // Convert the hour part to an integer for manipulation
    let mut hour: i32 = hour_str.parse().unwrap();

    // If AM, adjust the hour for 12 AM -> 00 hours
    if period == "AM" {
        if hour == 12 {
            hour = 0;
        }
    } else { // PM case
        if hour != 12 {
            hour += 12; // Convert to 24-hour format
        }
    }

    // Format the new time with leading zero if necessary
    format!("{:02}{}", hour, minute_and_second)
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
