use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|r| r.unwrap()).collect::<Vec<_>>();

    let numbers = lines.remove(0).split(',').map(|s| s.parse::<u8>().unwrap()).collect::<Vec<_>>();

    let mut boards = Vec::new();

    while !lines.is_empty() {
        // remove empty lines
        lines.remove(0);
        // dbg!(&lines);
        let mut board = [0_u8; 25];
        for i in 0..5_usize {
            let line = lines.remove(0);
            let numbers = line.split_whitespace().map(|s| s.parse::<u8>().unwrap());
            for (j, v) in numbers.enumerate() {
                board[(i * 5) + j] = v;
            }
        }
        boards.push(Board {
            index: boards.len(),
            board,
            revealed: [false; 25],
        });
    }

    println!("Loaded {} boards and {} drawn values!", boards.len(), numbers.len());

    for number in numbers {
        println!("Called {}", number);
        let mut complete_boards = Vec::new();
        for (i, board) in boards.iter_mut().enumerate() {
            if let Some(score) = board.reveal(number) {
                // insert at the start to reverse the order and remove issues when removing them
                complete_boards.insert(0, i);
            }
        }
        if boards.len() == 1 {
            let board = boards.first().unwrap();
            if board.has_won() {
                println!("Board {} looses with score {}", board.index, board.sum_score(number));
                return;
            }
        }
        for board in complete_boards {
            boards.remove(board);
        }
    }
}

#[derive(Debug)]
struct Board {
    index: usize,
    board: [u8; 25],
    revealed: [bool; 25],
}

impl Board {
    fn reveal(&mut self, number: u8) -> Option<u64> {
        for (i, v) in self.board.iter().enumerate() {
            if v == &number {
                self.revealed[i] = true;
            }
        }
        if self.has_won() {
            Some(self.sum_score(number))
        } else {
            None
        }
    }

    fn has_won(&self) -> bool {
        for i in 0..5_usize {
            let mut won = true;
            for j in 0..5_usize {
                if !self.revealed[(i * 5) + j] {
                    won = false;
                }
            }
            if won {
                return true;
            }
            let mut won = true;
            for j in 0..5_usize {
                if !self.revealed[(j * 5) + i] {
                    won = false;
                }
            }
            if won {
                return true;
            }
        }

        false
    }

    fn sum_score(&self, number: u8) -> u64 {
        let mut sum = 0_u64;
        for (v, revealed) in self.board.iter().zip(self.revealed.iter()) {
            if !revealed {
                sum += *v as u64;
            }
        }
        sum * (number as u64)
    }
}
