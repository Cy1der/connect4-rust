mod board;
mod game;
mod player;

use game::Game;
use player::Player;
use std::{io::stdin, process::exit};

fn main() {
    let player_one = Player::new(String::from("Player One"), 'X', 1);
    let player_two = Player::new(String::from("Player Two"), 'O', 2);
    let mut game = Game::new(player_one, player_two);

    // this is temporary

    game.board.print();

    loop {
        game.current_player.player_turn(&mut game.board);
        print!("{esc}c", esc = 27 as char);
        game.board.print();
        if let Some(_) = game.board.check_win(&game.current_player) {
            game.winner = Some(game.current_player.clone());
            break;
        }
        game.current_player = game.opponent();
    }

    println!("{} wins!", game.winner.unwrap().name);
    exit_game(0);
}

fn exit_game(code: i32) {
    println!("Press enter to exit...");
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    exit(code);
}
