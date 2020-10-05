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

    let mut players: HashMap<Player, Box<dyn Playable>> = HashMap::with_capacity(2);

    // Ready player one
    players.insert(Player::X, Box::new(ComputerPlayer::new(Player::X)));

    // Ready player two
    players.insert(Player::O, Box::new(ComputerPlayer::new(Player::O)));

    let mut current_player = Player::X;

    let outcome = loop {
        let player = &players[&current_player];

        let coordinate = player.play(&grid);
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
    fn play(&self, grid: &Grid) -> Coordinate;
}
