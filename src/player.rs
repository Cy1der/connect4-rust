use crate::board::{Board, BOARD_WIDTH};
use std::io::stdin;

#[derive(Clone)]
pub struct Player {
    pub name: String,
    pub symbol: char,
    pub id: i32,
}

impl Player {
    pub fn new(name: String, symbol: char, id: i32) -> Player {
        Player { name, symbol, id }
    }

    pub fn player_turn(&self, board: &mut Board) {
        let mut valid_input: bool = false;
        println!("{}'s turn", self.name);
        println!("Enter column number (1-{}):", board.width);
        while !valid_input {
            let mut column_choice: String = String::new();
            stdin()
                .read_line(&mut column_choice)
                .expect("Failed to read line");
            let column_choice: usize = match column_choice.trim().parse() {
                Ok(num) => match num {
                    1..=BOARD_WIDTH => {
                        valid_input = true;
                        num - 1
                    }
                    _ => {
                        println!("Please enter a valid column number (1-{})", board.width);
                        continue;
                    }
                },
                Err(_) => {
                    println!("Please enter a valid column number (1-{}).", board.width);
                    continue;
                }
            };

            if board.is_column_full(column_choice) {
                println!(
                    "Column {} is full. Please choose another column.",
                    column_choice + 1
                );
                continue;
            }

            board.update_board(column_choice, self);
            break;
        }
    }
}
