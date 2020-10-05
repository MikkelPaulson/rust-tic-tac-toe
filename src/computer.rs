use super::{Coordinate, Grid, Playable, Player};

pub struct ComputerPlayer {
    player: Player,
}

impl ComputerPlayer {
    pub fn new(player: Player) -> Self {
        Self { player }
    }
}

impl Playable for ComputerPlayer {
    fn play(&self, grid: &Grid) -> Coordinate {
        let mut legal_moves = Vec::with_capacity(9);
        for x in 0..=2 {
            for y in 0..=2 {
                let coordinate = Coordinate::new(x, y);
                if grid.is_legal(coordinate) {
                    legal_moves.push(coordinate);
                }
            }
        }

        println!("");
        println!("{} chooses {}", self.player, legal_moves[0]);
        println!("");

        legal_moves[0]
    }
}
