mod grid;
use std::fmt;
use std::io;

pub fn run() {
    let mut grid = grid::Grid::empty();
    let mut current_player = Player::X;

    println!("");

    loop {
        grid.draw();

        let coordinate: grid::Coordinate = loop {
            println!("");
            println!("Enter {} move:", current_player);

            let mut buf = String::new();
            io::stdin().read_line(&mut buf).unwrap();

            buf.pop(); // trim trailing newline
            match buf.parse() {
                Ok(coordinate) => {
                    println!("");
                    break coordinate;
                }
                Err(error) => println!("{}", error),
            };
        };

        current_player = current_player.turn();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn turn(&mut self) -> Self {
        match self {
            Self::X => Self::O,
            Self::O => Self::X,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X => write!(f, "X"),
            Self::O => write!(f, "O"),
        }
    }
}
