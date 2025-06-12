use crate::core::{Game, GameState, Player};
use crate::net::protocol::{ClientMessage, Message, ServerMessage, receive, send};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct Server {
    waiting_player: Option<TcpStream>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            waiting_player: None,
        }
    }

    pub fn run(&mut self) {
        let listener = TcpListener::bind("127.0.0.1:8080").expect("Server failed to start");
        println!("Server listening on 127.0.0.1:8080");

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            if self.waiting_player.is_none() {
                self.waiting_player = Some(stream);
                let mut waiting = self.waiting_player.as_mut().unwrap();
                send(&mut waiting, ServerMessage::WaitingOpponent);
                continue;
            }

            if let Some(player_a) = self.waiting_player.take() {
                let player_b = stream;
                thread::spawn(move || {
                    Self::handle_match(player_a, player_b);
                });
            }
        }
    }

    fn handle_match(mut player_a: TcpStream, mut player_b: TcpStream) {
        let mut game = Game::new();

        send(&mut player_a, ServerMessage::GameStarted);
        send(&mut player_b, ServerMessage::GameStarted);
        send(&mut player_a, ServerMessage::YouAreX);
        send(&mut player_b, ServerMessage::YouAreO);

        loop {
            let (current, other) = match game.current_player() {
                Player::X => (&mut player_a, &mut player_b),
                Player::O => (&mut player_b, &mut player_a),
            };

            send(current, ServerMessage::YourTurn);
            send(other, ServerMessage::OpponentTurn);

            match receive(current) {
                Message::Client(ClientMessage::PlayPosition(position)) => {
                    game.play(position);
                    send(other, ClientMessage::PlayPosition(position));

                    match game.game_state() {
                        GameState::InProgress => { /* keep looping */ }
                        GameState::Tie => {
                            send(&mut player_a, ServerMessage::YouTied);
                            send(&mut player_b, ServerMessage::YouTied);
                        }
                        GameState::Won(winner) => {
                            match winner {
                                Player::X => {
                                    send(&mut player_a, ServerMessage::YouWon);
                                    send(&mut player_b, ServerMessage::YouLost);
                                }
                                Player::O => {
                                    send(&mut player_a, ServerMessage::YouLost);
                                    send(&mut player_b, ServerMessage::YouWon);
                                }
                            }
                            break;
                        }
                    }
                }
                _ => panic!("Unexpected message from client"),
            }
        }
    }
}
