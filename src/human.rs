use super::{Coordinate, Grid, Playable, Player};
use std::io;

pub struct HumanPlayer {
    player: Player,
}

impl HumanPlayer {
    pub fn new(player: Player) -> Self {
        Self { player }
    }
}

impl Playable for HumanPlayer {
    fn play(&self, grid: &Grid) -> Coordinate {
        loop {
            println!("{}", grid);
            println!("");
            println!("Enter {} move:", self.player);

            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();

            buf.pop(); // trim trailing newline
            match buf.parse() {
                Ok(coordinate) => match grid.try_legal(coordinate) {
                    Ok(()) => break coordinate,
                    Err(e) => println!("{}", e),
                },
                Err(e) => println!("{}", e),
            };
        }
    }
}
