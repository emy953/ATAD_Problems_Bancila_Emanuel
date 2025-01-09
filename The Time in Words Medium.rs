use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeInWords' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts following parameters:
 *  1. INTEGER h
 *  2. INTEGER m
 */

fn timeInWords(h: i32, m: i32) -> String {
    let num_to_word = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        "ten", "eleven", "twelve", "thirteen", "fourteen", "fifteen", "sixteen", "seventeen", 
        "eighteen", "nineteen", "twenty", "twenty one", "twenty two", "twenty three", "twenty four", 
        "twenty five", "twenty six", "twenty seven", "twenty eight", "twenty nine", "thirty"
    ];

    if m == 0 {
        return format!("{} o' clock", num_to_word[h as usize]);
    } else if m == 15 {
        return format!("quarter past {}", num_to_word[h as usize]);
    } else if m == 30 {
        return format!("half past {}", num_to_word[h as usize]);
    } else if m == 45 {
        return format!("quarter to {}", num_to_word[(h % 12 + 1) as usize]);
    } else if m < 30 {
        let minute_word = if m == 1 { "minute" } else { "minutes" };
        return format!("{} {} past {}", num_to_word[m as usize], minute_word, num_to_word[h as usize]);
    } else {
        let minute_word = if 60 - m == 1 { "minute" } else { "minutes" };
        return format!("{} {} to {}", num_to_word[(60 - m) as usize], minute_word, num_to_word[((h % 12) + 1) as usize]);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let h = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let m = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = timeInWords(h, m);

    writeln!(&mut fptr, "{}", result).ok();
}
