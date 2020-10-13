mod computer;
mod game;
mod human;

use computer::ComputerPlayer;
use game::{Coordinate, Grid, Player};
use human::HumanPlayer;

pub fn run() {
    // Ready player one
    let player_x = ComputerPlayer::new(Player::X);

    // Ready player two
    let player_o = ComputerPlayer::new(Player::O);

    let final_grid = play(Box::new(player_x), Box::new(player_o));

    println!("");

    if let Some(winner) = final_grid.get_winner() {
        println!("{} wins!", winner);
    } else {
        println!("The game ended in a draw!");
    }

    println!("");
    println!("{}", final_grid);
}

fn play(mut player_x: Box<dyn Playable>, mut player_o: Box<dyn Playable>) -> Grid {
    let mut grid = Grid::empty();

    let mut current_player = Player::X;

    while grid.is_in_progress() {
        let coordinate = match current_player {
            Player::X => player_x.play(&grid),
            Player::O => player_o.play(&grid),
        };

        grid.set_space(&coordinate, &current_player)
            .expect("Illegal move!");

        current_player = current_player.turn();
    };

    grid
}

trait Playable {
    fn play(&mut self, grid: &Grid) -> Coordinate;
}
