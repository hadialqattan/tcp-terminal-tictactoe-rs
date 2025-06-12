use crate::core::Position;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub fn send<T: Signal>(stream: &mut TcpStream, signal: T) {
    let byte = signal.to_byte();
    stream
        .write_all(&[byte])
        .expect(&format!("Couldn't send signal: 0x{:02x}", byte));
}

pub fn receive(stream: &mut TcpStream) -> Message {
    let mut buffer = [0u8; 1];
    stream
        .read_exact(&mut buffer)
        .expect("Couldn't read signal");
    let byte = buffer[0];

    if let Some(signal) = ServerMessage::from_byte(byte) {
        return Message::Server(signal);
    }
    if let Some(signal) = ClientMessage::from_byte(byte) {
        return Message::Client(signal);
    }

    panic!("Invalid signal received: 0x{:02x}", byte);
}

pub trait Signal {
    fn to_byte(&self) -> u8;
    fn from_byte(byte: u8) -> Option<Self>
    where
        Self: Sized;
}

pub enum Message {
    Server(ServerMessage),
    Client(ClientMessage),
}

pub enum ServerMessage {
    WaitingOpponent,
    GameStarted,
    YouAreX,
    YouAreO,
    YourTurn,
    OpponentTurn,
    YouWon,
    YouLost,
    YouTied,
}

impl Signal for ServerMessage {
    fn to_byte(&self) -> u8 {
        match self {
            Self::WaitingOpponent => 0x11,
            Self::GameStarted => 0x12,
            Self::YouAreX => 0x13,
            Self::YouAreO => 0x14,
            Self::YourTurn => 0x15,
            Self::OpponentTurn => 0x16,
            Self::YouWon => 0x17,
            Self::YouLost => 0x18,
            Self::YouTied => 0x19,
        }
    }

    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            0x11 => Some(Self::WaitingOpponent),
            0x12 => Some(Self::GameStarted),
            0x13 => Some(Self::YouAreX),
            0x14 => Some(Self::YouAreO),
            0x15 => Some(Self::YourTurn),
            0x16 => Some(Self::OpponentTurn),
            0x17 => Some(Self::YouWon),
            0x18 => Some(Self::YouLost),
            0x19 => Some(Self::YouTied),
            _ => None,
        }
    }
}

#[derive(Clone)]
pub enum ClientMessage {
    PlayPosition(Position),
}

impl Signal for ClientMessage {
    fn to_byte(&self) -> u8 {
        match self {
            Self::PlayPosition(pos) => match pos {
                Position::One => 0x21,
                Position::Two => 0x22,
                Position::Three => 0x23,
                Position::Four => 0x24,
                Position::Five => 0x25,
                Position::Six => 0x26,
                Position::Seven => 0x27,
                Position::Eight => 0x28,
                Position::Nine => 0x29,
            },
        }
    }

    fn from_byte(byte: u8) -> Option<Self> {
        let position = match byte {
            0x21 => Position::One,
            0x22 => Position::Two,
            0x23 => Position::Three,
            0x24 => Position::Four,
            0x25 => Position::Five,
            0x26 => Position::Six,
            0x27 => Position::Seven,
            0x28 => Position::Eight,
            0x29 => Position::Nine,
            _ => return None,
        };
        Some(Self::PlayPosition(position))
    }
}
