use crate::core::{Board, Cell, Player, Position};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, PartialEq)]
pub enum GameState {
    InProgress,
    Won(Player),
    Tie,
}

pub struct Game {
    board: Board,
    current_player: Player,
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        // NOTE: This is to avoid using the external rand crate.
        //      Not recommended for serious applications.
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let first_player = if secs & 1 == 1 { Player::X } else { Player::O };

        Game {
            board: Board::new(),
            current_player: first_player,
            state: GameState::InProgress,
        }
    }

    pub fn play(&mut self, position: Position) {
        if self.state != GameState::InProgress {
            return;
        }

        let cell = Cell::from_player(self.current_player);
        self.board.play(position, cell);

        self.update_game_state();

        if self.state == GameState::InProgress {
            self.next_turn();
        }
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn game_state(&self) -> GameState {
        self.state
    }

    fn update_game_state(&mut self) {
        if self.has_winner() {
            self.state = GameState::Won(self.current_player);
        } else if self.board.is_full() {
            self.state = GameState::Tie;
        }
    }

    fn has_winner(&self) -> bool {
        #[rustfmt::skip]
        let [
            [one, two, three],
            [four, five, six],
            [seven, eight, nine]
        ] = *self.board.cells();

        #[rustfmt::skip]
        let lines = [
            [one, two, three], [four, five, six], [seven, eight, nine],
            [one, four, seven], [two, five, eight], [three, six, nine],
            [one, five, nine], [three, five, seven]
        ];

        lines
            .iter()
            .any(|&[x, y, z]| x == y && y == z && x != Cell::Empty)
    }

    fn next_turn(&mut self) {
        self.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
    }
}
