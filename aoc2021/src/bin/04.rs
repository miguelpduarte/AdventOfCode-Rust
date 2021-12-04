#[derive(Debug, Clone)]
struct BoardNumber {
    value: u16,
    line: u16,
    col: u16,
    called: bool,
}

impl BoardNumber {
    fn new(value: u16, line: u16, col: u16) -> Self {
        Self {
            value,
            line,
            col,
            called: false,
        }
    }
}

#[derive(Debug, Clone)]
struct Board {
    numbers: Vec<BoardNumber>,
    already_won: bool,
}

impl Board {
    fn new(board_numbers: Vec<BoardNumber>) -> Self {
        Self {
            numbers: board_numbers,
            already_won: false,
        }
    }

    fn mark_called_and_check_win(&mut self, called: u16) -> bool {
        if self.already_won {
            return false;
        }

        // Find called number
        let found_num_idx = self.numbers.iter().position(|b_num| b_num.value == called);

        match found_num_idx {
            Some(found_num_idx) => {
                // Get the found item and mark it as called
                self.numbers[found_num_idx].called = true;

                // Get a ref to the called number to use in the following lines
                let found_num: &BoardNumber = self.numbers.get(found_num_idx).unwrap();

                // Check if this board won using the called number's col and line
                // Check for col
                let col_all_called = self
                    .numbers
                    .iter()
                    .filter(|b_num| b_num.col == found_num.col)
                    .all(|b_num| b_num.called);
                // Check for line
                let line_all_called = self
                    .numbers
                    .iter()
                    .filter(|b_num| b_num.line == found_num.line)
                    .all(|b_num| b_num.called);

                if col_all_called || line_all_called {
                    self.already_won = true;
                }

                // This does not short circuit I guess (running both iterators?) but is more
                // elegant than checking either straight away I guess
                col_all_called || line_all_called
            }
            // Called number not in board, so can't win
            None => false,
        }
    }

    fn calc_score(&self, last_called: u16) -> u16 {
        let sum_unmarked_numbers: u16 = self
            .numbers
            .iter()
            .filter(|bn| !bn.called)
            .map(|bn| bn.value)
            .sum();
        sum_unmarked_numbers * last_called
    }
}

fn solve_day(input: String) -> (usize, usize) {
    let mut input_lines = input.lines();

    let numbers = input_lines
        .next()
        .unwrap()
        .split(',')
        .map(str::parse::<u16>)
        .map(Result::unwrap)
        .collect::<Vec<_>>();

    // Remove useless empty line
    input_lines.next().unwrap();

    let mut boards = Vec::<Board>::new();
    let mut board_numbers_acc = Vec::<BoardNumber>::new();
    let mut line = 0;

    for input_line in input_lines {
        match input_line {
            "" => {
                // Empty line, create board from currently accumulated items and reset variables
                let board = Board::new(board_numbers_acc);
                boards.push(board);

                board_numbers_acc = Vec::<BoardNumber>::new();
                line = 0;
            }
            input_line => {
                let mut line_boardnumbers = input_line
                    .split_whitespace()
                    .map(str::parse::<u16>)
                    .map(Result::unwrap)
                    .enumerate()
                    .map(|(col, val)| BoardNumber::new(val, line, col as u16))
                    .collect::<Vec<BoardNumber>>();
                board_numbers_acc.append(&mut line_boardnumbers);
                line += 1;
            }
        }
    }

    let board = Board::new(board_numbers_acc);
    boards.push(board);

    let mut final_score = 0;
    let mut boards_p1 = boards.clone();

    'outer: for number in numbers.clone() {
        for board in boards_p1.iter_mut() {
            if board.mark_called_and_check_win(number) {
                final_score = board.calc_score(number);
                break 'outer;
            }
        }
    }

    let p1 = final_score as usize;

    let n_boards = boards.len();
    let mut n_won = 0;
    let mut score = 0;

    'outer2: for number in numbers {
        for board in boards.iter_mut() {
            if board.mark_called_and_check_win(number) {
                n_won += 1;
                if n_won == n_boards {
                    score = board.calc_score(number);
                    break 'outer2;
                }
            }
        }
    }

    let p2 = score as usize;

    (p1, p2)
}

#[test]
fn example_input() {
    let input = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
        .to_owned();
    let res = solve_day(input);
    assert_eq!(res.0, 4512);
    assert_eq!(res.1, 1924);
}

#[test]
fn prod_solution() {
    use std::fs::read_to_string;

    let input = read_to_string(format!("inputs/{}", "4.in")).unwrap();
    let res = solve_day(input);
    assert_eq!(res.0, 60368);
    assert_eq!(res.1, 17435);
}

aoc2021::day_main!("4.in");
