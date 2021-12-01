use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();

    let mut last = None;
    let mut increases = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        let current = line.parse::<i64>().unwrap();
        if let Some(last) = last {
            if last < current {
                increases += 1;
            }
        }
        last = Some(current);
    }

    println!("Increased {} times!", increases);
}
