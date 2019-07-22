use std::collections::HashMap;
use std::str::from_utf8_mut;

#[derive(PartialEq, Eq, Debug)]
enum Colors {
    Black,
    White,
}

#[derive(Debug)]
struct Board {
    cells: [[char; 8]; 8],
    half_move: u8,
    full_move: u32,
    side: Colors,
    is_white_long_change: bool,
    is_white_short_change: bool,
    is_black_long_change: bool,
    is_black_short_change: bool,
    double_cell: String,
}

impl Board {
    fn new() -> Board {
        Board {
            cells:
            [
                ['.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.'],
                ['.', '.', '.', '.', '.', '.', '.', '.'],
            ],
            half_move: 0,
            full_move: 0,
            side: Colors::White,
            is_white_long_change: true,
            is_white_short_change: true,
            is_black_long_change: true,
            is_black_short_change: true,
            double_cell: "-".to_string(),
        }
    }

    /**
    Загрузка фигур из fen нотации.
    */
    fn load_fen_line(&mut self, fen_line: &str) {

        self.is_black_long_change = false;
        self.is_black_short_change = false;
        self.is_white_long_change = false;
        self.is_white_short_change = false;

        let (figures, other) = fen_line
            .split_at(fen_line
                .find(" ")
                .expect("need to enter correct line"));
        let lines: Vec<&str> = figures.split("/").collect();
        let mut counter = 0;
        for (i, line) in lines.iter().enumerate() {
            for c in line.chars().into_iter() {
                if c.is_digit(10) {
                    for k in 0..c.to_digit(10).unwrap() {
                        self.cells[i][counter] = '.';
                        counter += 1;
                    }
                } else {
                    self.cells[i][counter] = c;
                    counter += 1;
                }
            }
            counter = 0;
        }

        for (i, action) in other.split_whitespace().enumerate() {
            match i {
                0 => if action == "b" {
                    println!("Black turn");
                    self.side = Colors::Black
                } else {
                    println!("White turn");
                    self.side = Colors::White
                }
                1 => {
                    if action == "-" {
                        self.is_black_long_change = false;
                        self.is_black_short_change = false;
                        self.is_white_long_change = false;
                        self.is_white_short_change = false;
                    } else {
                        for state in action.chars().into_iter() {
                            match state {
                                'K' => { self.is_white_short_change = true }
                                'Q' => { self.is_white_long_change = true }
                                'k' => { self.is_black_short_change = true }
                                'q' => { self.is_black_long_change = true }
                                _ => {}
                            }
                        }
                    }
                }
                2 => { self.double_cell = action.to_string() }
                3 => { self.half_move = action.to_string().parse().expect("Expect number in half move part.") }
                4 => { self.full_move = action.to_string().parse().expect("Expect number in full move part.") }
                _ => { println!("Incorrect line format") }
            }
        }
    }

    /**
    Обработка ходов фигур.
    */
    fn make_move(&mut self, action: &str) {
        let mut row_from = 0;
        let mut col_from = 0;
        let mut row_to = 0;
        let mut col_to = 0;

        for (i, column) in action.chars().enumerate() {
            match i {
                0 => { row_from = get_row(column) as usize }
                1 => { col_from = 7 - (column.to_digit(10).expect("Expect digit in second symbol of move") as usize - 1) }
                2 => { row_to = get_row(column) as usize }
                3 => { col_to = 7 - (column.to_digit(10).expect("Expect digit in second symbol of move") as usize - 1) }
                _ => {}
            }
        }

        let figure = self.cells[col_from][row_from];
        let victim = self.cells[col_to][row_to];
        let size = (col_from as i32 - col_to as i32);

        if (figure == 'p' || figure == 'P') && victim == '.' && size.abs() == 2 {
            if size < 0 {
                self.double_cell = action.chars().into_iter().next().unwrap().to_string() + (8 - col_from - 1).to_string().as_str();
            } else {
                self.double_cell = action.chars().into_iter().next().unwrap().to_string() + (8 - col_from + 1).to_string().as_str();
            }
        }

        if figure == 'k' {
            self.is_black_short_change = false;
            self.is_black_long_change = false;
        } else if figure == 'K' {
            self.is_white_short_change = false;
            self.is_white_long_change = false;
        }

        let horizontal_move = row_from as i8 - row_to as i8;

        if figure == 'k' &&
            victim == '.' &&
            col_from == col_to &&
            horizontal_move.abs() == 2 {
            if size < 0 && self.cells[col_from][7] == 'r' {
                self.cells[col_from][7] = '.';
                self.cells[col_from][row_to - 1] = 'r';
            } else if size > 0 && self.cells[col_from][0] == 'r' {
                self.cells[col_from][0] = '.';
                self.cells[col_from][row_to + 1] = 'r';
            }

            self.is_black_long_change = false;
            self.is_black_short_change = false;
        }

        if figure == 'K' &&
            victim == '.' &&
            col_from == col_to &&
            size.abs() == 2 {
            if size < 0 && self.cells[col_from][7] == 'R' {
                self.cells[col_from][7] = '.';
                self.cells[col_from][row_to - 1] = 'R';
            } else if size > 0 && self.cells[col_from][0] == 'R' {
                self.cells[col_from][0] = '.';
                self.cells[col_from][row_to + 1] = 'R';
            }

            self.is_white_long_change = false;
            self.is_white_short_change = false;
        }



        self.cells[col_from][row_from] = '.';
        self.cells[col_to][row_to] = figure;


        if (figure == 'p' || figure == 'P') || victim != '.' {
            self.half_move = 0;
        } else {
            self.half_move += 1;
        }

        if self.side == Colors::Black {
            self.full_move += 1;
            self.side = Colors::White;
        } else {
            self.side = Colors::Black;
        }
    }

    /**
    Рисуем доску в консоли
    */
    fn draw_board(&self) {
        println!("  +-----------------+");
        for (i, line) in self.cells.iter().enumerate() {
            print!("{} | ", 8 - i);
            for figure in line.iter() {
                print!("{} ", figure)
            }
            println!("|");
        }
        println!("  +-----------------+");
        println!("    a b c d e f g h ");
    }

    fn get_fen_string(&self) -> String {
        let mut result_string = String::new();
        let mut counter: u8 = 0;
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                if self.cells[i][j] == '.' {
                    counter += 1;
                } else {
                    if counter > 0 {
                        result_string.push_str(counter.to_string().as_str());
                        counter = 0;
                    }
                    result_string.push_str(self.cells[i][j].to_string().as_str());
                }
            }
            if counter > 0 {
                result_string.push_str(counter.to_string().as_str());
            }
            counter = 0;
            if i < self.cells.len() - 1 {
                result_string.push_str("/");
            }
        }

        result_string.push_str(" ");
        result_string.push_str(match &self.side {
            Colors::Black => "b",
            Colors::White => "w",
        });

        result_string.push_str(" ");
        if self.is_white_long_change == false &&
            self.is_white_short_change == false &&
            self.is_black_long_change == false &&
            self.is_black_short_change == false {
            result_string.push_str("-");
        } else {
            result_string.push_str(if self.is_white_long_change == true { "K" } else { "" });
            result_string.push_str(if self.is_white_short_change == true { "Q" } else { "" });
            result_string.push_str(if self.is_black_long_change == true { "k" } else { "" });
            result_string.push_str(if self.is_black_short_change == true { "q" } else { "" });
        }

        result_string.push_str(" ");
        result_string.push_str(self.double_cell.as_str());
        result_string.push_str(" ");

        result_string.push_str(self.half_move.to_string().as_str());
        result_string.push_str(" ");
        result_string.push_str(self.full_move.to_string().as_str());
        result_string
    }
}

fn get_row(c: char) -> u8 {
    match c {
        'a' => { 0 }
        'b' => { 1 }
        'c' => { 2 }
        'd' => { 3 }
        'e' => { 4 }
        'f' => { 5 }
        'g' => { 6 }
        'h' => { 7 }
        _ => { 10 }
    }
}


fn main() {
    let mut board = Board::new();
    board.load_fen_line("r2k3r/pppppppp/8/N7/8/8/PPPPPPPP/R3K2R w KQ - 1 17");
    board.draw_board();
    board.make_move("e1d1");
    println!("e1d1");
    board.draw_board();
    println!("{}", board.get_fen_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_fen_line() {
        let mut board = Board::new();
        board.load_fen_line("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KkQq - 0 1");
        assert_eq!(board.get_fen_string(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    }

    #[test]
    fn test_change_side() {
        let mut board = Board::new();
        board.load_fen_line("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        board.make_move("e2e2");
        assert_eq!(board.get_fen_string(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 1");
        board.make_move("e2e2");
        assert_eq!(board.get_fen_string(), "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 2");
    }

    #[test]
    fn test_change_half_move_counter() {
        let mut board = Board::new();
        board.load_fen_line("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        board.make_move("b1f3");
        assert_eq!(board.get_fen_string(), "rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/R1BQKBNR b KQkq - 1 1");

        board.load_fen_line("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        board.make_move("e2e4");
        assert_eq!(board.get_fen_string(), "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1");

        board.load_fen_line("rnbqkb1r/pppppppp/8/4N3/6n1/8/PPPPPPPP/RNBQKB1R w KQkq - 4 3");
        board.make_move("e5g4");
        assert_eq!(board.get_fen_string(), "rnbqkb1r/pppppppp/8/8/6N1/8/PPPPPPPP/RNBQKB1R b KQkq - 0 3");

        board.load_fen_line("rnbqkb1r/pppppppp/8/4N3/6n1/8/PPPPPPPP/RNBQKB1R w KQkq - 4 3");
        board.make_move("b1f3");
        assert_eq!(board.get_fen_string(), "rnbqkb1r/pppppppp/8/4N3/6n1/5N2/PPPPPPPP/R1BQKB1R b KQkq - 5 3");
    }

    #[test]
    fn test_king_move_drop_flag_roker() {
        let mut board = Board::new();
        board.load_fen_line("r3k2r/pppppppp/8/N7/8/8/PPPPPPPP/R3K2R b KQkq - 0 16");
        board.make_move("e8d8");
        assert_eq!(board.get_fen_string(), "r2k3r/pppppppp/8/N7/8/8/PPPPPPPP/R3K2R w KQ - 1 17");

        board.load_fen_line("r2k3r/pppppppp/8/N7/8/8/PPPPPPPP/R3K2R w KQ - 1 17");
        board.make_move("e1d1");
        assert_eq!(board.get_fen_string(), "r2k3r/pppppppp/8/N7/8/8/PPPPPPPP/R2K3R b - - 2 17");

        board.load_fen_line("r3k2r/pppppppp/8/n7/2N5/8/PPPPPPPP/R3K2R w KQkq - 4 16");
        board.make_move("e1d1");
        assert_eq!(board.get_fen_string(), "r3k2r/pppppppp/8/n7/2N5/8/PPPPPPPP/R2K3R b kq - 5 16");
    }
}