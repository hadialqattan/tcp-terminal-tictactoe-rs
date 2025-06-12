use core::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Cell {
    X,
    O,
    Empty,
}

impl Cell {
    pub fn from_player(player: Player) -> Self {
        match player {
            Player::X => Cell::X,
            Player::O => Cell::O,
        }
    }
}

#[derive(Clone, Copy)]
#[rustfmt::skip]
pub enum Position {
    One, Two, Three,
    Four, Five, Six,
    Seven, Eight, Nine
}

impl Position {
    pub fn coords(&self) -> (usize, usize) {
        match self {
            Position::One => (0, 0),
            Position::Two => (0, 1),
            Position::Three => (0, 2),
            Position::Four => (1, 0),
            Position::Five => (1, 1),
            Position::Six => (1, 2),
            Position::Seven => (2, 0),
            Position::Eight => (2, 1),
            Position::Nine => (2, 2),
        }
    }

    pub fn from_number(x: i32) -> Self {
        match x {
            1 => Position::One,
            2 => Position::Two,
            3 => Position::Three,
            4 => Position::Four,
            5 => Position::Five,
            6 => Position::Six,
            7 => Position::Seven,
            8 => Position::Eight,
            9 => Position::Nine,
            _ => panic!("Position value should be from 1 to 9."),
        }
    }
}
