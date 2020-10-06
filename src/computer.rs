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
        grid.set_space(coordinate, player).ok(); // This is okay.
        grid
    }

    fn forking_move(
        &self,
        grid: &Grid,
        legal_moves: &Vec<Coordinate>,
        player: &Player,
    ) -> Option<Coordinate> {
        for coordinate in legal_moves {
            let mut fork_count = 0;
            let next_grid = self.try_move(grid, coordinate, player);
            for next_coordinate in legal_moves {
                if next_coordinate != coordinate {
                    if Some(player)
                        == self
                            .try_move(&next_grid, next_coordinate, player)
                            .get_winner()
                            .as_ref()
                    {
                        fork_count += 1;
                    }
                }
            }
            if fork_count > 1 {
                println!("{} can set up a fork by playing {}", player, coordinate);
                return Some(*coordinate);
            }
        }
        None
    }

    fn winning_move(
        &self,
        grid: &Grid,
        legal_moves: &Vec<Coordinate>,
        player: &Player,
    ) -> Option<Coordinate> {
        for coordinate in legal_moves {
            if self
                .try_move(grid, coordinate, player)
                .get_winner()
                .is_some()
            {
                println!("{} can win by playing {}", player, coordinate);
                return Some(*coordinate);
            }
        }
        None
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

        legal_moves.shuffle(&mut self.rng);

        let coordinate = None
            // Can I make a winning move?
            .or_else(|| self.winning_move(&grid, &legal_moves, &self.player))
            // Can my opponent make a winning move?
            .or_else(|| self.winning_move(&grid, &legal_moves, &self.player.turn()))
            // Can I be cunning and fork the other player?
            .or_else(|| self.forking_move(&grid, &legal_moves, &self.player))
            // Can the other player make a legal move that will cause me to be forked?
            .or_else(|| self.forking_move(&grid, &legal_moves, &self.player.turn()))
            // Just make a random move
            .or_else(|| legal_moves.first().cloned())
            .expect("No legal moves!");

        println!("{} chooses {}", self.player, coordinate);
        println!("");

        coordinate
    }
}

#[cfg(test)]
mod test {
    use super::{ComputerPlayer, Coordinate, Grid, Playable, Player};

    #[test]
    fn takes_winning_move() {
        // X..
        // .*.
        // ..X
        let mut player = ComputerPlayer::new(Player::X);
        let mut grid = Grid::empty();
        grid.set_space(&Coordinate::new(0, 0), &Player::X).unwrap();
        grid.set_space(&Coordinate::new(2, 2), &Player::X).unwrap();
        assert_eq!(Coordinate::new(1, 1), player.play(&grid));

        // O..
        // .@.
        // ..O
        let mut player = ComputerPlayer::new(Player::O);
        let mut grid = Grid::empty();
        grid.set_space(&Coordinate::new(0, 0), &Player::O).unwrap();
        grid.set_space(&Coordinate::new(2, 2), &Player::O).unwrap();
        assert_eq!(Coordinate::new(1, 1), player.play(&grid));
    }

    #[test]
    fn blocks_winning_opponent() {
        // O..
        // .*.
        // ..O
        let mut player = ComputerPlayer::new(Player::X);
        let mut grid = Grid::empty();
        grid.set_space(&Coordinate::new(0, 0), &Player::O).unwrap();
        grid.set_space(&Coordinate::new(2, 2), &Player::O).unwrap();
        assert_eq!(Coordinate::new(1, 1), player.play(&grid));

        // X..
        // .@.
        // ..X
        let mut player = ComputerPlayer::new(Player::O);
        let mut grid = Grid::empty();
        grid.set_space(&Coordinate::new(0, 0), &Player::X).unwrap();
        grid.set_space(&Coordinate::new(2, 2), &Player::X).unwrap();
        assert_eq!(Coordinate::new(1, 1), player.play(&grid));
    }

    #[test]
    fn prefers_winning_move_over_blocking_opponent() {
        let mut grid = Grid::empty();
        grid.set_space(&Coordinate::new(0, 0), &Player::X).unwrap();
        grid.set_space(&Coordinate::new(0, 1), &Player::O).unwrap();
        grid.set_space(&Coordinate::new(1, 0), &Player::X).unwrap();
        grid.set_space(&Coordinate::new(1, 1), &Player::O).unwrap();

        // XX*
        // OO.
        // ...
        let mut player = ComputerPlayer::new(Player::X);
        assert_eq!(Coordinate::new(2, 0), player.play(&grid));

        // XX.
        // OO@
        // ...
        let mut player = ComputerPlayer::new(Player::O);
        assert_eq!(Coordinate::new(2, 1), player.play(&grid));
    }

    #[test]
    fn sets_up_fork() {
        // ..O
        // .*X
        // .OX
        let mut player = ComputerPlayer::new(Player::X);
        let mut grid = Grid::empty();
        grid.set_space(&Coordinate::new(2, 1), &Player::X).unwrap();
        grid.set_space(&Coordinate::new(2, 0), &Player::O).unwrap();
        grid.set_space(&Coordinate::new(2, 2), &Player::X).unwrap();
        grid.set_space(&Coordinate::new(1, 2), &Player::O).unwrap();
        assert_eq!(Coordinate::new(1, 1), player.play(&grid));
    }

    #[test]
    fn prefers_blocking_opponent_over_setting_up_fork() {
        // *OO
        // ..X
        // ..X
        let mut player = ComputerPlayer::new(Player::X);
        let mut grid = Grid::empty();
        grid.set_space(&Coordinate::new(2, 1), &Player::X).unwrap();
        grid.set_space(&Coordinate::new(2, 0), &Player::O).unwrap();
        grid.set_space(&Coordinate::new(2, 2), &Player::X).unwrap();
        grid.set_space(&Coordinate::new(1, 0), &Player::O).unwrap();
        assert_eq!(Coordinate::new(0, 0), player.play(&grid));
    }

    #[test]
    fn blocks_opposing_fork() {
        // OX.
        // X@.
        // ...
        let mut player = ComputerPlayer::new(Player::O);
        let mut grid = Grid::empty();
        grid.set_space(&Coordinate::new(0, 1), &Player::X).unwrap();
        grid.set_space(&Coordinate::new(0, 0), &Player::O).unwrap();
        grid.set_space(&Coordinate::new(1, 0), &Player::X).unwrap();
        assert_eq!(Coordinate::new(1, 1), player.play(&grid));
    }

    #[test]
    fn prefers_setting_up_fork_over_blocking_opposing_fork() {
        // *X.
        // X.O
        // .O@
        let mut grid = Grid::empty();
        grid.set_space(&Coordinate::new(0, 1), &Player::X).unwrap();
        grid.set_space(&Coordinate::new(2, 1), &Player::O).unwrap();
        grid.set_space(&Coordinate::new(1, 0), &Player::X).unwrap();
        grid.set_space(&Coordinate::new(1, 2), &Player::O).unwrap();

        let mut player = ComputerPlayer::new(Player::X);
        assert_eq!(Coordinate::new(0, 0), player.play(&grid));

        let mut player = ComputerPlayer::new(Player::O);
        assert_eq!(Coordinate::new(2, 2), player.play(&grid));
    }

    #[test]
    fn moves_at_random() {
        let mut player_x = ComputerPlayer::new(Player::X);
        let mut player_o = ComputerPlayer::new(Player::O);

        let mut grids = [Grid::empty(), Grid::empty()];

        for i in 0..=1 {
            while grids[i].get_winner() == None && grids[i].has_legal_moves() {
                grids[i]
                    .set_space(&player_x.play(&grids[i]), &Player::X)
                    .unwrap();
                if grids[i].get_winner().is_some() || !grids[i].has_legal_moves() {
                    break;
                }
                grids[i]
                    .set_space(&player_o.play(&grids[i]), &Player::O)
                    .unwrap();
            }
        }

        // Each game should have a different outcome.
        assert_ne!(grids[0], grids[1]);
    }
}
