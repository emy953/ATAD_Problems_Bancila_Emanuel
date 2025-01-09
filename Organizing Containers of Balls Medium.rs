use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'organizingContainers' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts 2D_INTEGER_ARRAY container as parameter.
 */

fn organizingContainers(container: &[Vec<i32>]) -> String {
    let mut row_sums = Vec::new();
    let mut column_sums = Vec::new();

    // Calculate row sums and store them in `row_sums`
    for row in container.iter() {
        let row_sum: i32 = row.iter().sum(); // Sum all elements in the row
        row_sums.push(row_sum);
    }

    // Calculate column sums and store them in `column_sums`
    if let Some(first_row) = container.get(0) {
        for j in 0..first_row.len() {
            let column_sum: i32 = container.iter().map(|row| row[j]).sum();
            column_sums.push(column_sum);
        }
    }

    for x in row_sums.iter()
    {
        let mut ok = 0;
        for y in column_sums.iter()
            {
                if x==y
                {
                    ok=1;
                }
            }
        if ok==0
        {
        return "Impossible".to_string();
        }
    }
    return "Possible".to_string();

}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let q = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    for _ in 0..q {
        let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

        let mut container: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

        for i in 0..n as usize {
            container.push(Vec::with_capacity(n as usize));

            container[i] = stdin_iterator.next().unwrap().unwrap()
                .trim_end()
                .split(' ')
                .map(|s| s.to_string().parse::<i32>().unwrap())
                .collect();
        }

        let result = organizingContainers(&container);

        writeln!(&mut fptr, "{}", result).ok();
    }
}
