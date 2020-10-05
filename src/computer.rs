use super::{Coordinate, Grid, Playable, Player};
use rand::prelude::*;

pub struct ComputerPlayer {
    player: Player,
    rng: ThreadRng,
}

impl ComputerPlayer {
    pub fn new(player: Player) -> Self {
        Self {
            player,
            rng: thread_rng(),
        }
    }
}

impl Playable for ComputerPlayer {
    fn play(&mut self, grid: &Grid) -> Coordinate {
        let mut legal_moves = Vec::with_capacity(9);
        for x in 0..=2 {
            for y in 0..=2 {
                let coordinate = Coordinate::new(x, y);
                if grid.is_legal(&coordinate) {
                    legal_moves.push(coordinate);
                }
            }
        }

        let coordinate = legal_moves.choose(&mut self.rng).expect("No legal moves!");

        println!("");
        println!("{} chooses {}", self.player, coordinate);
        println!("");

        *coordinate
    }
}
