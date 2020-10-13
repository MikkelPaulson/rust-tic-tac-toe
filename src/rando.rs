use super::{Coordinate, Grid, Playable};
use rand::prelude::*;

pub struct RandoPlayer {
    rng: ThreadRng,
}

impl RandoPlayer {
    pub fn new() -> Self {
        Self { rng: thread_rng() }
    }
}

impl Playable for RandoPlayer {
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

        *legal_moves.choose(&mut self.rng).expect("No legal moves!")
    }
}
