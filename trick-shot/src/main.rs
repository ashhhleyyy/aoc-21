use std::{ops::RangeInclusive, str::FromStr, io::{BufRead, Write}};

fn main() {
    let target = std::io::stdin().lock().lines().next().map(Result::unwrap).map(|s| Target::from_str(&s)).unwrap().unwrap();
    let mut max_y = 0;
    let mut count_within = 0;
    for x_vel in -500..=500 {
        for y_vel in -500..=500 {
            if let Some(new_max_y) = simulate_launch((x_vel, y_vel), &target) {
                count_within += 1;
                if new_max_y > max_y {
                    max_y = new_max_y;
                    print!("\r[{},{}] max-y={}     ", x_vel, y_vel, max_y);
                    std::io::stdout().flush().unwrap();
                }
            }
        }
    }
    println!();
    println!("max_y={}", max_y);
    println!("count_within={}", count_within);
}

fn simulate_launch(initial_velocity: P, target: &Target) -> Option<i64> {
    let mut current_position: P = (0, 0);
    let mut current_velocity = initial_velocity;
    let mut max_y = 0;
    let mut i = 0;
    loop {
        i += 1;
        current_position.0 += current_velocity.0;
        current_position.1 += current_velocity.1;
        if current_velocity.0 > 0 {
            current_velocity.0 -= 1;
        } else if current_velocity.0 < 0 {
            current_velocity.0 += 1;
        }
        current_velocity.1 -= 1;

        if current_position.1 > max_y {
            max_y = current_position.1;
        }

        if target.contains(&current_position) {
            return Some(max_y);
        }

        if target.is_past(&current_position) {
            return None;
        }
    }
}

type P = (i64, i64);

struct Target {
    x: RangeInclusive<i64>,
    y: RangeInclusive<i64>,
}

impl Target {
    fn contains(&self, pos: &P) -> bool {
        self.x.contains(&pos.0) && self.y.contains(&pos.1)
    }

    fn is_past(&self, pos: &P) -> bool {
        pos.1 < *self.y.start()
    }
}

impl FromStr for Target {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("target area: ").unwrap();
        let (x, y) = s.split_once(", ").unwrap();
        let (x, y) = (x.strip_prefix("x=").unwrap(), y.strip_prefix("y=").unwrap());
        let (x1, x2) = x.split_once("..").unwrap();
        let (x1, x2) = (x1.parse::<i64>().unwrap(), x2.parse::<i64>().unwrap());
        let (y1, y2) = y.split_once("..").unwrap();
        let (y1, y2) = (y1.parse::<i64>().unwrap(), y2.parse::<i64>().unwrap());
        Ok(Self {
            x: x1..=x2,
            y: y1..=y2,
        })
    }
}
