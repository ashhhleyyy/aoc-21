use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();

    let mut values = Vec::new();

    for line in stdin.lock().lines() {
        values.push(line.unwrap().parse::<i64>().unwrap());
    }

    let mut increases = 0;
    let mut last = None;

    for i in 0..(values.len() - 2) {
        let current_total = values[i] + values[i+1] + values[i+2];
        if let Some(last) = last {
            if last < current_total {
                increases += 1;
            }
        }
        last = Some(current_total);
    }

    println!("{} increases!", increases);
}
