#[derive(Debug)]
struct Board([[u8; 5]; 5]);

#[derive(Debug, Default, Copy, Clone)]
struct BoardState([[bool; 5]; 5]);

impl From<&[&str]> for Board {
    fn from(lines: &[&str]) -> Self {
        // Discard blank line
        let lines = &lines[1..];

        let mut rows = [[0; 5]; 5];
        for (row_index, line) in lines.iter().enumerate() {
            for (column_index, num) in line
                .split(' ')
                .filter_map(|num| num.parse().ok())
                .enumerate()
            {
                rows[row_index][column_index] = num;
            }
        }

        Self(rows)
    }
}

impl Board {
    fn mark_number(&self, number: u8, state: &mut BoardState) {
        for (row_index, row) in self.0.iter().enumerate() {
            for (column_index, cell) in row.iter().enumerate() {
                if *cell == number {
                    state.0[row_index][column_index] = true;
                    return;
                }
            }
        }
    }

    fn score(&self, number: u8, state: &BoardState) -> u64 {
        let mut sum = 0;

        for (row_index, row) in self.0.iter().enumerate() {
            for (column_index, &cell) in row.iter().enumerate() {
                if !state.0[row_index][column_index] {
                    sum += cell as u64;
                }
            }
        }

        sum * number as u64
    }
}

impl BoardState {
    fn has_won(&self) -> bool {
        // Check all rows
        for row in self.0 {
            if row.iter().all(|&i| i) {
                return true;
            }
        }

        // Check all columns
        for column_index in 0..5 {
            if self.0.iter().all(|row| row[column_index]) {
                return true;
            }
        }

        false
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut lines = input.lines();

    let numbers: Vec<u8> = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|num| num.parse().ok())
        .collect();

    let lines: Vec<&str> = lines.collect();
    let boards: Vec<Board> = lines.chunks_exact(6).map(From::from).collect();
    let mut board_states: Vec<BoardState> = vec![Default::default(); boards.len()];

    'outer: for number in numbers {
        for (board, board_state) in boards.iter().zip(&mut board_states) {
            board.mark_number(number, board_state);

            if board_state.has_won() {
                println!("part1 = {}", board.score(number, board_state));
                break 'outer;
            }
        }
    }
}
