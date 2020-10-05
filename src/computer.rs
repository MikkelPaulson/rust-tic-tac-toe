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

impl ComputerPlayer {
    fn try_move(&self, grid: &Grid, coordinate: &Coordinate, player: &Player) -> Grid {
        let mut grid = grid.clone();
        grid.set_space(coordinate, player);
        grid
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

        let coordinate = None
            .or_else(|| {
                for coordinate in &legal_moves {
                    if let Some(_) = self.try_move(&grid, &coordinate, &self.player).get_winner() {
                        return Some(coordinate);
                    }
                }
                None
            })
            .or_else(|| {
                let other_player = self.player.turn();
                for coordinate in &legal_moves {
                    if let Some(_) = self
                        .try_move(&grid, &coordinate, &other_player)
                        .get_winner()
                    {
                        return Some(coordinate);
                    }
                }
                None
            })
            .or_else(|| legal_moves.choose(&mut self.rng))
            .expect("No legal moves!");

        println!("{} chooses {}", self.player, coordinate);
        println!("");

        *coordinate
    }
}
