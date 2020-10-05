mod computer;
mod game;
mod human;

use computer::ComputerPlayer;
use game::{Coordinate, Grid, Player};
use human::HumanPlayer;
use std::boxed::Box;
use std::collections::HashMap;
use std::io;

pub fn run() {
    let mut grid = Grid::empty();

    // Ready player one
    let mut player_x = ComputerPlayer::new(Player::X);

    // Ready player two
    let mut player_o = ComputerPlayer::new(Player::O);

    let mut current_player = Player::X;

    let outcome = loop {
        let coordinate = match current_player {
            Player::X => &mut player_x,
            Player::O => &mut player_o,
        }
        .play(&grid);

        grid.set_space(&coordinate, &current_player)
            .expect("Illegal move!");

        if let Some(winner) = grid.get_winner() {
            break Some(winner);
        } else if !grid.has_legal_moves() {
            break None;
        }

        current_player = current_player.turn();
    };

    println!("");

    if let Some(winner) = outcome {
        println!("{} wins!", winner);
    } else {
        println!("The game ended in a draw!");
    }

    println!("");
    println!("{}", grid);
}

trait Playable {
    fn play(&mut self, grid: &Grid) -> Coordinate;
}
