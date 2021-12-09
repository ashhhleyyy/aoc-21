use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();

    let mut count = 0;

    for line in stdin.lock().lines().map(Result::unwrap) {
        let (unique_values, output_digits) = line.split_once(" | ").unwrap();
        let (unique_values, output_digits) = (unique_values.split(" "), output_digits.splitn(4, " "));
        for digit in output_digits {
            let l = digit.len() as i32;
            if l == 2 || l == 3 || l == 4 || l == 7 {
                count += 1;
            }
        }
    }

    println!("{} unique value digits", count);
}
