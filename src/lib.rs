mod game;
mod human;

use game::{Coordinate, Grid, Player};
use std::boxed::Box;
use std::collections::HashMap;
use std::io;

pub fn run() {
    let mut grid = Grid::empty();

    let mut players: HashMap<Player, Box<dyn Playable>> = HashMap::with_capacity(2);

    // Ready player one
    players.insert(Player::X, Box::new(human::HumanPlayer::new(Player::X)));

    // Ready player two
    players.insert(Player::O, Box::new(human::HumanPlayer::new(Player::O)));

    let mut current_player = Player::X;

    let winner = loop {
        let player = &players[&current_player];

        let coordinate = player.play(&grid);
        grid.set_space(coordinate, current_player)
            .expect("Illegal move!");
        if let Some(winner) = grid.get_winner() {
            break winner;
        }

        current_player = current_player.turn();
    };

    println!("");
    println!("{} wins!", winner);
    println!("");
    println!("{}", grid);
}

trait Playable {
    fn play(&self, grid: &Grid) -> Coordinate;
}
