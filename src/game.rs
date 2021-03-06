use std::fmt;
use std::iter;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    spaces: [[Space; 3]; 3],
}

impl Grid {
    pub fn new(spaces: [[Space; 3]; 3]) -> Grid {
        Grid { spaces }
    }

    pub fn empty() -> Grid {
        Grid::new([
            [Space::Empty, Space::Empty, Space::Empty],
            [Space::Empty, Space::Empty, Space::Empty],
            [Space::Empty, Space::Empty, Space::Empty],
        ])
    }

    pub fn get_space(&self, coordinate: &Coordinate) -> Space {
        self.spaces[coordinate.1][coordinate.0]
    }

    pub fn is_legal(&self, coordinate: &Coordinate) -> bool {
        self.get_space(coordinate).get_player() == None
    }

    pub fn try_legal(&self, coordinate: &Coordinate) -> Result<(), IllegalMove> {
        if self.is_legal(coordinate) {
            Ok(())
        } else {
            Err(IllegalMove(*coordinate))
        }
    }

    pub fn set_space(
        &mut self,
        coordinate: &Coordinate,
        player: &Player,
    ) -> Result<(), IllegalMove> {
        self.try_legal(coordinate)
            .map(|_| self.spaces[coordinate.1][coordinate.0] = Space::new(Some(player)))
    }

    pub fn lines(&self) -> LineIterator {
        LineIterator::new(self.spaces.clone())
    }

    pub fn get_winner(&self) -> Option<Player> {
        for line in self.lines() {
            if let Some(winner) = line.get_winner() {
                return Some(winner);
            }
        }
        None
    }

    pub fn has_legal_moves(&self) -> bool {
        for x in 0..=2 {
            for y in 0..=2 {
                if self.is_legal(&Coordinate::new(x, y)) {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_in_progress(&self) -> bool {
        self.has_legal_moves() && self.get_winner().is_none()
    }
}

impl fmt::Display for Grid {
    //     A   B   C
    //   +---+---+---+
    // 1 | X |   |   |
    //   +---+---+---+
    // 2 |   | X |   |
    //   +---+---+---+
    // 3 |   |   | O |
    //   +---+---+---+
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "     A   B   C")?;
        writeln!(f, "   +---+---+---+")?;

        let mut row_num = 0;
        for row in self.spaces.iter() {
            row_num = row_num + 1;
            writeln!(f, " {} | {} | {} | {} |", row_num, row[0], row[1], row[2])?;
            writeln!(f, "   +---+---+---+")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod test_grid {
    use super::{Coordinate, Grid, IllegalMove, Line, Player, Space};

    #[test]
    fn empty() {
        assert_eq!(
            Grid {
                spaces: [
                    [Space::Empty, Space::Empty, Space::Empty],
                    [Space::Empty, Space::Empty, Space::Empty],
                    [Space::Empty, Space::Empty, Space::Empty],
                ]
            },
            Grid::empty(),
        );
    }

    #[test]
    fn get_space() {
        assert_eq!(
            Space::X,
            Grid::new([
                [Space::Empty, Space::X, Space::Empty],
                [Space::Empty, Space::Empty, Space::Empty],
                [Space::Empty, Space::Empty, Space::Empty],
            ])
            .get_space(&Coordinate(1, 0)),
        );
    }

    #[test]
    fn is_legal() {
        let grid = Grid::new([
            [Space::Empty, Space::X, Space::Empty],
            [Space::Empty, Space::Empty, Space::Empty],
            [Space::Empty, Space::Empty, Space::Empty],
        ]);

        assert_eq!(true, grid.is_legal(&Coordinate(0, 1)));
        assert_eq!(false, grid.is_legal(&Coordinate(1, 0)));
    }

    #[test]
    fn set_space() {
        let mut grid = Grid::empty();
        assert_eq!(Ok(()), grid.set_space(&Coordinate(0, 2), &Player::X));
        assert_eq!(
            Err(IllegalMove(Coordinate(0, 2))),
            grid.set_space(&Coordinate(0, 2), &Player::O),
        );
        assert_eq!(Ok(()), grid.set_space(&Coordinate(2, 0), &Player::O));
    }

    #[test]
    fn lines() {
        assert_eq!(
            Some(Line([
                (Coordinate(0, 0), Space::O),
                (Coordinate(1, 0), Space::X),
                (Coordinate(2, 0), Space::O),
            ])),
            Grid::new([
                [Space::O, Space::X, Space::O],
                [Space::Empty, Space::Empty, Space::Empty],
                [Space::Empty, Space::Empty, Space::Empty],
            ])
            .lines()
            .next()
        );
    }

    #[test]
    fn get_winner() {
        let grid = Grid::new([
            [Space::O, Space::X, Space::X],
            [Space::Empty, Space::O, Space::Empty],
            [Space::Empty, Space::Empty, Space::X],
        ]);
        assert_eq!(None, grid.get_winner());
        assert!(grid.is_in_progress());

        let grid = Grid::new([
            [Space::O, Space::X, Space::X],
            [Space::Empty, Space::O, Space::Empty],
            [Space::Empty, Space::Empty, Space::O],
        ]);
        assert_eq!(Some(Player::O), grid.get_winner());
        assert!(!grid.is_in_progress());
    }

    #[test]
    fn has_legal_moves() {
        let grid = Grid::empty();
        assert!(grid.has_legal_moves());
        assert!(grid.is_in_progress());

        let grid = Grid::new([
            [Space::X, Space::X, Space::O],
            [Space::O, Space::Empty, Space::X],
            [Space::X, Space::O, Space::O],
        ]);
        assert!(grid.has_legal_moves());
        assert!(grid.is_in_progress());

        let grid = Grid::new([
            [Space::X, Space::X, Space::O],
            [Space::O, Space::X, Space::X],
            [Space::X, Space::O, Space::O],
        ]);
        assert!(!grid.has_legal_moves());
        assert!(!grid.is_in_progress());
    }

    #[test]
    fn display() {
        let grid = Grid::new([
            [Space::O, Space::X, Space::X],
            [Space::Empty, Space::X, Space::Empty],
            [Space::Empty, Space::Empty, Space::Empty],
        ]);

        assert_eq!(
            "     A   B   C
   +---+---+---+
 1 | O | X | X |
   +---+---+---+
 2 |   | X |   |
   +---+---+---+
 3 |   |   |   |
   +---+---+---+
",
            format!("{}", grid),
        );
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Line([(Coordinate, Space); 3]);

impl Line {
    pub fn get_winner(&self) -> Option<Player> {
        let spaces = &self.get_spaces();
        if spaces[0] == spaces[1] && spaces[0] == spaces[2] {
            spaces[0].get_player()
        } else {
            None
        }
    }

    fn get_spaces(&self) -> [Space; 3] {
        [self.0[0].1, self.0[1].1, self.0[2].1]
    }
}

#[cfg(test)]
mod test_line {
    use super::{Coordinate, Line, Player, Space};

    fn get_line(spaces: [Space; 3]) -> Line {
        Line([
            (Coordinate(0, 0), spaces[0]),
            (Coordinate(1, 0), spaces[1]),
            (Coordinate(2, 0), spaces[2]),
        ])
    }

    #[test]
    fn get_winner() {
        assert_eq!(None, get_line([Space::O, Space::O, Space::X]).get_winner());
        assert_eq!(None, get_line([Space::O, Space::X, Space::O]).get_winner());
        assert_eq!(None, get_line([Space::X, Space::O, Space::O]).get_winner());
        assert_eq!(
            None,
            get_line([Space::Empty, Space::Empty, Space::Empty]).get_winner(),
        );
        assert_eq!(
            Some(Player::X),
            get_line([Space::X, Space::X, Space::X]).get_winner(),
        );
        assert_eq!(
            Some(Player::O),
            get_line([Space::O, Space::O, Space::O]).get_winner(),
        );
    }
}

pub struct LineIterator {
    spaces: [[Space; 3]; 3],
    counter: usize,
}

impl LineIterator {
    const LINE_PROFILES: [[[usize; 2]; 3]; 8] = [
        [[0, 0], [1, 0], [2, 0]], // row 1
        [[0, 1], [1, 1], [2, 1]], // row 2
        [[0, 2], [1, 2], [2, 2]], // row 3
        [[0, 0], [0, 1], [0, 2]], // column A
        [[1, 0], [1, 1], [1, 2]], // column B
        [[2, 0], [2, 1], [2, 2]], // column C
        [[0, 0], [1, 1], [2, 2]], // diagonal \
        [[0, 2], [1, 1], [2, 0]], // diagonal /
    ];

    pub fn new(spaces: [[Space; 3]; 3]) -> LineIterator {
        LineIterator { spaces, counter: 0 }
    }
}

impl iter::Iterator for LineIterator {
    type Item = Line;

    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < Self::LINE_PROFILES.len() {
            let profile = Self::LINE_PROFILES[self.counter];
            let x = [profile[0][0], profile[1][0], profile[2][0]];
            let y = [profile[0][1], profile[1][1], profile[2][1]];
            self.counter += 1;

            Some(Line([
                (Coordinate(x[0], y[0]), self.spaces[y[0]][x[0]]),
                (Coordinate(x[1], y[1]), self.spaces[y[1]][x[1]]),
                (Coordinate(x[2], y[2]), self.spaces[y[2]][x[2]]),
            ]))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (8, Some(8))
    }
}

impl iter::ExactSizeIterator for LineIterator {}

#[cfg(test)]
mod test_line_iterator {
    use super::{Coordinate, Grid, Line, LineIterator, Space};

    #[test]
    fn test_iterator() {
        // This grid should be identifiably different for each line
        let mut iterator = LineIterator::new([
            [Space::O, Space::X, Space::X],
            [Space::Empty, Space::X, Space::Empty],
            [Space::Empty, Space::Empty, Space::Empty],
        ]);

        assert_eq!(
            Some(Line([
                (Coordinate(0, 0), Space::O),
                (Coordinate(1, 0), Space::X),
                (Coordinate(2, 0), Space::X),
            ])),
            iterator.next(),
        );
        assert_eq!(
            Some(Line([
                (Coordinate(0, 1), Space::Empty),
                (Coordinate(1, 1), Space::X),
                (Coordinate(2, 1), Space::Empty),
            ])),
            iterator.next(),
        );
        assert_eq!(
            Some(Line([
                (Coordinate(0, 2), Space::Empty),
                (Coordinate(1, 2), Space::Empty),
                (Coordinate(2, 2), Space::Empty),
            ])),
            iterator.next(),
        );
        assert_eq!(
            Some(Line([
                (Coordinate(0, 0), Space::O),
                (Coordinate(0, 1), Space::Empty),
                (Coordinate(0, 2), Space::Empty),
            ])),
            iterator.next(),
        );
        assert_eq!(
            Some(Line([
                (Coordinate(1, 0), Space::X),
                (Coordinate(1, 1), Space::X),
                (Coordinate(1, 2), Space::Empty),
            ])),
            iterator.next(),
        );
        assert_eq!(
            Some(Line([
                (Coordinate(2, 0), Space::X),
                (Coordinate(2, 1), Space::Empty),
                (Coordinate(2, 2), Space::Empty),
            ])),
            iterator.next(),
        );
        assert_eq!(
            Some(Line([
                (Coordinate(0, 0), Space::O),
                (Coordinate(1, 1), Space::X),
                (Coordinate(2, 2), Space::Empty),
            ])),
            iterator.next(),
        );
        assert_eq!(
            Some(Line([
                (Coordinate(0, 2), Space::Empty),
                (Coordinate(1, 1), Space::X),
                (Coordinate(2, 0), Space::X),
            ])),
            iterator.next(),
        );
        assert_eq!(None, iterator.next());
        assert_eq!(None, iterator.next());
    }

    #[test]
    fn test_exact_size_iterator() {
        assert_eq!(8, Grid::empty().lines().len());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coordinate(usize, usize);

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Coordinate {
        Coordinate(x, y)
    }
}

impl FromStr for Coordinate {
    type Err = ParseCoordinateError;

    fn from_str(raw: &str) -> Result<Self, Self::Err> {
        if raw.len() != 2 {
            Err(ParseCoordinateError(raw.to_string()))
        } else {
            Ok(Coordinate(
                match raw.chars().nth(0) {
                    Some('A') | Some('a') => 0,
                    Some('B') | Some('b') => 1,
                    Some('C') | Some('c') => 2,
                    _ => return Err(ParseCoordinateError(raw.to_string())),
                },
                match raw.chars().nth(1) {
                    Some('1') => 0,
                    Some('2') => 1,
                    Some('3') => 2,
                    _ => return Err(ParseCoordinateError(raw.to_string())),
                },
            ))
        }
    }
}

impl From<Coordinate> for String {
    fn from(coordinate: Coordinate) -> String {
        format!("{}", coordinate)
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            0 => write!(f, "A")?,
            1 => write!(f, "B")?,
            2 => write!(f, "C")?,
            _ => unreachable!(),
        };
        match self.1 {
            0 => write!(f, "1")?,
            1 => write!(f, "2")?,
            2 => write!(f, "3")?,
            _ => unreachable!(),
        };
        Ok(())
    }
}

#[cfg(test)]
mod test_coordinate {
    use super::{Coordinate, ParseCoordinateError};

    #[test]
    fn from_str_valid() {
        assert_eq!(
            Ok(Coordinate(0, 0)),
            "A1".parse::<Coordinate>(),
            "{:?}",
            "A1",
        );
        assert_eq!(
            Ok(Coordinate(0, 1)),
            "a2".parse::<Coordinate>(),
            "{:?}",
            "a2",
        );
        assert_eq!(
            Ok(Coordinate(1, 1)),
            "B2".parse::<Coordinate>(),
            "{:?}",
            "B2",
        );
        assert_eq!(
            Ok(Coordinate(1, 2)),
            "b3".parse::<Coordinate>(),
            "{:?}",
            "b3",
        );
        assert_eq!(
            Ok(Coordinate(2, 2)),
            "C3".parse::<Coordinate>(),
            "{:?}",
            "C3",
        );
        assert_eq!(
            Ok(Coordinate(2, 0)),
            "c1".parse::<Coordinate>(),
            "{:?}",
            "c1",
        );
    }

    #[test]
    fn from_str_invalid() {
        assert_eq!(
            Err(ParseCoordinateError("".to_string())),
            "".parse::<Coordinate>(),
            "{:?}",
            "",
        );
        assert_eq!(
            Err(ParseCoordinateError("D1".to_string())),
            "D1".parse::<Coordinate>(),
            "{:?}",
            "D1",
        );
        assert_eq!(
            Err(ParseCoordinateError("A4".to_string())),
            "A4".parse::<Coordinate>(),
            "{:?}",
            "A4",
        );
        assert_eq!(
            Err(ParseCoordinateError("A12".to_string())),
            "A12".parse::<Coordinate>(),
            "{:?}",
            "A12",
        );
    }

    #[test]
    fn into_string() {
        assert_eq!("A2", &String::from(Coordinate(0, 1)));
        assert_eq!("B3", &String::from(Coordinate(1, 2)));
        assert_eq!("C1", &String::from(Coordinate(2, 0)));
    }

    #[test]
    fn display() {
        assert_eq!("A3", format!("{}", Coordinate(0, 2)));
        assert_eq!("B1", format!("{}", Coordinate(1, 0)));
        assert_eq!("C2", format!("{}", Coordinate(2, 1)));
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseCoordinateError(String);

impl fmt::Display for ParseCoordinateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid coordinate: {} (expected format: A1)", self.0)
    }
}

#[cfg(test)]
mod test_parse_coordinate_error {
    use super::ParseCoordinateError;

    #[test]
    fn format() {
        assert_eq!(
            &format!("{}", ParseCoordinateError("foo".to_string())),
            "Invalid coordinate: foo (expected format: A1)"
        );
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct IllegalMove(Coordinate);

impl fmt::Display for IllegalMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not a legal move.", self.0)
    }
}

#[cfg(test)]
mod test_illegal_move {
    use super::{Coordinate, IllegalMove};

    #[test]
    fn display() {
        assert_eq!(
            "A2 is not a legal move.",
            &format!("{}", IllegalMove(Coordinate(0, 1))),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn turn(&self) -> Self {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Space {
    X,
    O,
    Empty,
}

impl Space {
    pub fn new(player: Option<&Player>) -> Space {
        match player {
            Some(Player::X) => Space::X,
            Some(Player::O) => Space::O,
            None => Space::Empty,
        }
    }

    pub fn get_player(&self) -> Option<Player> {
        match self {
            Space::X => Some(Player::X),
            Space::O => Some(Player::O),
            Space::Empty => None,
        }
    }
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Space::X => write!(f, "X"),
            Space::O => write!(f, "O"),
            Space::Empty => write!(f, " "),
        }
    }
}

#[cfg(test)]
mod test_space {
    use super::{Player, Space};

    #[test]
    fn new() {
        assert_eq!(Space::X, Space::new(Some(&Player::X)));
        assert_eq!(Space::O, Space::new(Some(&Player::O)));
        assert_eq!(Space::Empty, Space::new(None));
    }

    #[test]
    fn get_player() {
        assert_eq!(Some(Player::X), Space::X.get_player());
        assert_eq!(Some(Player::O), Space::O.get_player());
        assert_eq!(None, Space::Empty.get_player());
    }

    #[test]
    fn display() {
        assert_eq!("X", &format!("{}", Space::X));
        assert_eq!("O", &format!("{}", Space::O));
        assert_eq!(" ", &format!("{}", Space::Empty));
    }
}
