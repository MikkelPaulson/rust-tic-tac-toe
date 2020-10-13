mod computer;
mod game;
mod human;
mod rando;

use computer::ComputerPlayer;
use game::{Coordinate, Grid, Player};
use human::HumanPlayer;
use rando::RandoPlayer;

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
    }

    grid
}

trait Playable {
    fn play(&mut self, grid: &Grid) -> Coordinate;
}

#[cfg(test)]
mod test_play {
    use super::*;

    #[test]
    #[ignore]
    fn computer_playing_x() {
        let mut x_wins = 0;
        let mut o_wins = 0;
        let mut draws = 0;

        for _ in 0..1000 {
            let grid = play(
                Box::new(ComputerPlayer::new_silent(Player::X)),
                Box::new(RandoPlayer::new()),
            );

            match grid.get_winner() {
                Some(Player::X) => x_wins += 1,
                Some(Player::O) => o_wins += 1,
                None => draws += 1,
            }
        }

        assert_eq!(
            0, o_wins,
            "Computer should never lose as X; actual outcome {} wins, {} losses, {} draws",
            x_wins, o_wins, draws
        );
    }

    #[test]
    #[ignore]
    fn computer_playing_o() {
        let mut x_wins = 0;
        let mut o_wins = 0;
        let mut draws = 0;

        for _ in 0..1000 {
            let grid = play(
                Box::new(RandoPlayer::new()),
                Box::new(ComputerPlayer::new_silent(Player::O)),
            );

            match grid.get_winner() {
                Some(Player::X) => x_wins += 1,
                Some(Player::O) => o_wins += 1,
                None => draws += 1,
            }
        }

        assert_eq!(
            0, x_wins,
            "Computer should never lose as O; actual outcome {} wins, {} losses, {} draws",
            o_wins, x_wins, draws
        );
    }
}
