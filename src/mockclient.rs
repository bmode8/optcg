use std::io::stdin;
use std::sync::mpsc::{Receiver, Sender};

use super::game::*;
use super::player::*;
use super::*;

pub struct MockPlayerClient {
    pub this_player: Box<Player>,
    pub other_player: Box<Player>,
    pub public_playfield_state: Box<PublicPlayfieldState>,
    pub tx: Sender<PlayerAction>,
    pub rx: Receiver<ServerMessage>,
}

impl MockPlayerClient {
    pub fn handle_messages(&mut self) {
        while let Ok(message) = self.rx.try_recv() {
            match message {
                ServerMessage::Connected => {}
                ServerMessage::RequestDeck => {}
                ServerMessage::QueryMulligan => {
                    self.respond_to_query_mulligan();
                }
                ServerMessage::TakeMainAction => {
                    self.respond_to_take_main_action();
                }
                ServerMessage::PlayerDataPayload(player) => {
                    self.this_player = player;
                }
                ServerMessage::OtherPlayerDataPayload(player) => {
                    self.other_player = player;
                }
                ServerMessage::PublicPlayfieldStateDataPayload(state) => {
                    self.public_playfield_state = state;
                }
            }
        }
    }

    pub fn respond_to_query_mulligan(&self) {
        println!("Hand: ");
        for card in self.this_player.hand.iter() {
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
        for (i, card) in self.this_player.hand.iter().enumerate() {
            println!("{i}\n{:?}", card);
        }

        // FIXME: No don??? Actually though the client is going to need to know a version of the board state, so
        // maybe the best way to handle that is adding some of the `PlayField` data to the `MockPlayerClient` struct?
        // Now is also about the time to integrate the FaceDown/FaceUp status throughout the turn.

        println!("Action: ");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let main_action = parse_main_action(input.trim().to_lowercase().as_str());

        match main_action {
            PlayerAction::End => {
                self.tx.send(PlayerAction::End).unwrap();
            }
            _ => self.respond_to_take_main_action(),
        }
    }
}

fn parse_main_action(input: &str) -> PlayerAction {
    use PlayerAction::*;

    let cleaned_input = input.trim().to_lowercase();
    let cleaned_input = cleaned_input.as_str();

    if cleaned_input == "" {
        return NoAction;
    }

    let words: Vec<_> = cleaned_input.split_whitespace().collect();

    match words[0] {
        "help" => {
            println!("These are the following commands you can use during the main phase:");
            println!("help - Show this help message.");
            println!("hand - Show your hand.");
            println!("board - Examine the current board state.");
            println!("examine <place> <card number> - Examine a card that is in your hand or face up on the board for its full text.");
            println!("play <card number> - Play a card from your hand.");
            println!("activate <card number> - Activate a card effect on the board.");
            println!("attach <card number> - Attach a DON!! card from your hand.");
            println!(
                "battle <card number> - Initiate a battle with a character from the board. 
                      Your leader is represented by 'L' instead of a number."
            );
            println!("end - End your turn.");
            NoAction
        }
        "end" => End,
        _ => NoAction,
    }
}
