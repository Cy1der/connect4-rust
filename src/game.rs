use crate::{board::Board, player::Player};

pub struct Game {
    pub board: Board,
    pub player_one: Player,
    pub player_two: Player,
    pub current_player: Player,
    pub winner: Option<Player>,
}

impl Game {
    pub fn new(player_one: Player, player_two: Player) -> Game {
        Game {
            board: Board::new(),
            player_one: player_one.clone(),
            player_two,
            current_player: player_one,
            winner: None,
        }
    }

    pub fn opponent(&self) -> Player {
        if self.current_player.id == self.player_one.id {
            self.player_two.clone()
        } else {
            self.player_one.clone()
        }
    }
}
