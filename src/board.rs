use crate::player::Player;

const BOARD_HEIGHT: usize = 6;
pub const BOARD_WIDTH: usize = 7;
const EMPTY_SPACE: char = ' ';

#[derive(Clone, Copy)]
pub struct Board {
    pub board: [[char; BOARD_HEIGHT]; BOARD_WIDTH],
    pub width: usize,
    pub height: usize,
}

pub enum WinOptions {
    Player1Win,
    Player2Win,
    Draw,
}

impl Board {
    pub fn new() -> Board {
        return Board {
            board: [[EMPTY_SPACE; BOARD_HEIGHT]; BOARD_WIDTH],
            width: BOARD_WIDTH,
            height: BOARD_HEIGHT,
        };
    }

    pub fn print(&self) {
        print!("╔");
        for i in 0..BOARD_WIDTH {
            print!(
                "═{}═{}",
                i + 1,
                if i == BOARD_WIDTH - 1 { "╗" } else { "╦" }
            );
        }
        println!();
        for row in 0..BOARD_HEIGHT {
            print!("║");
            for col in 0..BOARD_WIDTH {
                print!(" {} ║", self.board[col][row]);
            }
            println!();
        }
        println!("╚{}═══╝", "═══╩".repeat(BOARD_WIDTH - 1));
    }

    // pub fn print_raw(&self) {
    //     for row in 0..BOARD_HEIGHT {
    //         for col in 0..BOARD_WIDTH {
    //             print!("{}", self.board[col][row]);
    //         }
    //         println!();
    //     }
    //     for i in 1..=BOARD_WIDTH {
    //         print!("{}", i);
    //     }
    //     println!();
    // }

    pub fn check_win(&self, player: &Player) -> Option<WinOptions> {
        if self
            .board
            .iter()
            .all(|row| row.iter().all(|&space| space != EMPTY_SPACE))
        {
            return Some(WinOptions::Draw);
        }

        // if player 1 wins, return 1
        // if player 2 wins, return -1

        let horizontal = self.check_horizontal_win(player);
        let vertical = self.check_vertical_win(player);
        let diagonal = self.check_diagonal_win(player);

        if horizontal.0 {
            return match horizontal.1 {
                'X' => Some(WinOptions::Player1Win),
                'O' => Some(WinOptions::Player2Win),
                _ => None,
            };
        }

        if vertical.0 {
            return match vertical.1 {
                'X' => Some(WinOptions::Player1Win),
                'O' => Some(WinOptions::Player2Win),
                _ => None,
            };
        }

        if diagonal.0 {
            return match diagonal.1 {
                'X' => Some(WinOptions::Player1Win),
                'O' => Some(WinOptions::Player2Win),
                _ => None,
            };
        }

        return None;
    }

    pub fn check_horizontal_win(&self, player: &Player) -> (bool, char) {
        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH - 3 {
                if self.board[col][row] == player.symbol
                    && self.board[col + 1][row] == player.symbol
                    && self.board[col + 2][row] == player.symbol
                    && self.board[col + 3][row] == player.symbol
                {
                    return (true, player.symbol);
                }
            }
        }
        (false, EMPTY_SPACE)
    }

    pub fn check_vertical_win(&self, player: &Player) -> (bool, char) {
        for col in 0..BOARD_WIDTH {
            for row in 0..BOARD_HEIGHT - 3 {
                if self.board[col][row] == player.symbol
                    && self.board[col][row + 1] == player.symbol
                    && self.board[col][row + 2] == player.symbol
                    && self.board[col][row + 3] == player.symbol
                {
                    return (true, player.symbol);
                }
            }
        }
        (false, EMPTY_SPACE)
    }

    pub fn check_diagonal_win(&self, player: &Player) -> (bool, char) {
        for row in 0..BOARD_HEIGHT - 3 {
            for col in 0..BOARD_WIDTH - 3 {
                if self.board[col][row] == player.symbol
                    && self.board[col + 1][row + 1] == player.symbol
                    && self.board[col + 2][row + 2] == player.symbol
                    && self.board[col + 3][row + 3] == player.symbol
                {
                    return (true, player.symbol);
                }
            }
        }
        for row in 0..BOARD_HEIGHT - 3 {
            for col in 3..BOARD_WIDTH {
                if self.board[col][row] == player.symbol
                    && self.board[col - 1][row + 1] == player.symbol
                    && self.board[col - 2][row + 2] == player.symbol
                    && self.board[col - 3][row + 3] == player.symbol
                {
                    return (true, player.symbol);
                }
            }
        }
        (false, EMPTY_SPACE)
    }

    pub fn update_board(&mut self, col: usize, player: &Player) {
        for row in (0..self.height).rev() {
            if self.board[col][row] == EMPTY_SPACE {
                self.board[col][row] = player.symbol;
                break;
            }
        }
    }

    pub fn is_column_full(&self, col: usize) -> bool {
        for row in 0..self.height {
            if self.board[col][row] == EMPTY_SPACE {
                return false;
            }
        }
        return true;
    }

    // pub fn is_winning_move(board: &Board, player: &Player) -> bool {
    //     let is_winning_move = Board::check_win(board, player);
    //     match is_winning_move {
    //         Some(_) => true,
    //         None => false,
    //     }
    // }

    // pub fn score(&self, player: &Player) -> i32 {
    //     let is_winning_move = Board::is_winning_move(self, player);
    //     if is_winning_move {
    //         return 10;
    //     } else {
    //         return -10;
    //     }
    // }
}
