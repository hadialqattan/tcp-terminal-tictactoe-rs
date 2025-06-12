use crate::core::{Board, Cell, Player, Position};
use crate::net::protocol::{ClientMessage, Message, ServerMessage, receive, send};
use std::io::{self, Write};
use std::net::TcpStream;

pub struct Client {
    board: Option<Board>,
    role: Option<Player>,
}

impl Client {
    pub fn new() -> Self {
        Client {
            board: None,
            role: None,
        }
    }

    pub fn run(&mut self) {
        let mut stream = TcpStream::connect("127.0.0.1:8080").expect("Client failed to connect");
        println!("Client connected to server!");

        loop {
            match receive(&mut stream) {
                Message::Server(signal) => match signal {
                    ServerMessage::WaitingOpponent => {
                        println!("Waiting for an opponent to join...");
                    }
                    ServerMessage::GameStarted => {
                        self.board = Some(Board::new());
                        print!("{}", self.board.as_ref().unwrap());
                    }
                    ServerMessage::YouAreX => {
                        println!("You are X");
                        self.role = Some(Player::X);
                    }
                    ServerMessage::YouAreO => {
                        println!("You are O");
                        self.role = Some(Player::O);
                    }
                    ServerMessage::YourTurn => {
                        print!("Play (enter a number from 1 to 9): ");
                        io::stdout().flush().unwrap();

                        let mut input = String::new();
                        io::stdin()
                            .read_line(&mut input)
                            .expect("Failed to read input ;(");

                        let position_number = input
                            .trim()
                            .parse::<i32>()
                            .expect("Invalid: input should be a number ;(");

                        let position = Position::from_number(position_number);
                        send(&mut stream, ClientMessage::PlayPosition(position));

                        let cell = Cell::from_player(self.role.unwrap());
                        self.board.as_mut().unwrap().play(position, cell);
                        print!("{}", self.board.as_ref().unwrap());
                    }
                    ServerMessage::OpponentTurn => {
                        println!("It's your opponent's turn, please wait...")
                    }
                    ServerMessage::YouWon => {
                        println!("You won!!");
                        break;
                    }
                    ServerMessage::YouLost => {
                        println!("You lost!!");
                        break;
                    }
                    ServerMessage::YouTied => {
                        println!("You tied!!");
                        break;
                    }
                },
                Message::Client(ClientMessage::PlayPosition(position)) => {
                    let cell = match self.role.unwrap() {
                        Player::X => Cell::from_player(Player::O),
                        Player::O => Cell::from_player(Player::X),
                    };
                    self.board.as_mut().unwrap().play(position, cell);
                    print!("{}", self.board.as_ref().unwrap());
                }
            }
        }
    }
}
