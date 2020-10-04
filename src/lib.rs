mod game;

use game::{Grid, Player};
use std::io;

pub fn run() {
    let mut grid = Grid::empty();
    let mut current_player = Player::X;

    let winner = loop {
        println!("");
        println!("{}", grid);

        loop {
            println!("");
            println!("Enter {} move:", current_player);

            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();

            buf.pop(); // trim trailing newline
            match buf.parse() {
                Ok(coordinate) => match grid.set_space(coordinate, current_player) {
                    Ok(()) => break,
                    Err(e) => println!("{}", e),
                },
                Err(e) => println!("{}", e),
            };
        }

        current_player = current_player.turn();

        if let Some(winner) = grid.get_winner() {
            break winner;
        }
    };

    println!("");
    println!("{} wins!", winner);
    println!("");
    println!("{}", grid);
}
