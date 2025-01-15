use std::io::stdin;
use std::sync::mpsc::{Receiver, Sender};

use serde::{Deserialize, Serialize};

use super::player::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerAction {
    TakeMulligan,
    NoAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    QueryMulligan,
    TakeMainAction,
    PlayerDataPayload(Box<Player>),
}

pub struct MockPlayerClient {
    pub player: Box<Player>,
    pub tx: Sender<PlayerAction>,
    pub rx: Receiver<ServerMessage>,
}

impl MockPlayerClient {
    pub fn handle_message(&mut self) {
        while let Ok(message) = self.rx.try_recv() {
            match message {
                ServerMessage::QueryMulligan => {
                    self.respond_to_query_mulligan();
                }
                ServerMessage::TakeMainAction => {
                    self.respond_to_take_main_action();
                }
                ServerMessage::PlayerDataPayload(player) => {
                    self.player = player;
                }
            }
        }
    }

    pub fn respond_to_query_mulligan(&self) {
        println!("Hand: ");
        for card in self.player.hand.iter() {
            println!("{:?}", card);
        }
        println!("Mulligan? [y/N]  ");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        match input.trim().to_lowercase().as_str() {
            "y" => self.tx.send(PlayerAction::TakeMulligan).unwrap(),
            "n" | "" => self.tx.send(PlayerAction::NoAction).unwrap(),
            _ => self.respond_to_query_mulligan(),
        }
    }

    pub fn respond_to_take_main_action(&self) {
        println!("Hand: ");
        for (i, card) in self.player.hand.iter().enumerate() {
            println!("{i}\n{:?}", card);
        }

        // FIXME: No don??? Actually though the client is going to need to know a version of the board state, so
        // maybe the best way to handle that is adding some of the `PlayField` data to the `MockPlayerClient` struct?
        // Now is also about the time to integrate the FaceDown/FaceUp status throughout the turn.

        println!("Action: ");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let cleaned_input = input.trim().to_lowercase();
        let cleaned_input = cleaned_input.as_str();

        match cleaned_input {
            "end" => {
                self.tx.send(PlayerAction::NoAction).unwrap();
            }
            _ => self.respond_to_take_main_action(),
        }
    }
}
