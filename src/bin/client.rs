#![allow(unused)]
use std::io::stdin;

use futures::prelude::*;
use serde_json::Value;
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio_serde::formats::*;
use tokio_serde::Framed;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use optcg::{PlayerAction, ServerMessage};
use optcg::player::*;
use optcg::game::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    let mut client = Client::new(Box::new(Player::empty()), Box::new(Player::empty()), Box::new(PublicPlayfieldState::empty()), &mut stream);
    
    loop {
        client.handle_messages().await;
        client.send_action(PlayerAction::Idle).await;

        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
}

struct Client<'stream> {
    this_player: Box<Player>,
    other_player: Box<Player>,
    public_playfield_state: Box<PublicPlayfieldState>,
    writer: Framed<FramedWrite<WriteHalf<'stream>, LengthDelimitedCodec>, Value, Value, Json<Value, Value>>,
    reader: Framed<FramedRead<ReadHalf<'stream>, LengthDelimitedCodec>, Value, Value, Json<Value, Value>>,
}

impl<'stream> Client<'stream> {
    pub fn new(this_player: Box<Player>, other_player: Box<Player>, public_playfield_state: Box<PublicPlayfieldState>, socket: &'stream mut TcpStream) -> Self {
        let (rx, tx) = socket.split();

        let read_frame = FramedRead::new(rx, LengthDelimitedCodec::new());
        let reader = tokio_serde::SymmetricallyFramed::new(read_frame, SymmetricalJson::<Value>::default());
        let write_frame = FramedWrite::new(tx, LengthDelimitedCodec::new());
        let writer = tokio_serde::SymmetricallyFramed::new(write_frame, SymmetricalJson::<Value>::default());
        Self {
            this_player,
            other_player,
            public_playfield_state,
            writer,
            reader,
        }
    }

    pub async fn handle_messages(&mut self) {
        while let Ok(message) = serde_json::from_value::<ServerMessage>(self.reader.try_next().await.unwrap().unwrap()) {
            match message {
                ServerMessage::Connected => { 
                    self.send_action(PlayerAction::Idle).await;
                }
                ServerMessage::RequestDeck => { }
                ServerMessage::QueryMulligan => {
                    println!("Hand: ");
                    for card in self.this_player.hand.iter() {
                        println!("{:?}", card);
                    }
                    self.respond_to_query_mulligan().await;
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

    pub async fn respond_to_query_mulligan(&mut self) {
        println!("Mulligan? [y/N]  ");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        loop {
            match input.trim().to_lowercase().as_str() {
                "y" => self.send_action(PlayerAction::TakeMulligan).await,
                "n" | "" => self.send_action(PlayerAction::NoAction).await,
                _ => continue,
            }
        }
    }

    pub async fn send_action(&mut self, action: PlayerAction) {
        self.writer.send(serde_json::from_str(serde_json::to_string(&action).unwrap().as_str()).unwrap()).await.unwrap();
    }

    pub fn respond_to_take_main_action(&mut self) {
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
                self.send_action(PlayerAction::End);
            }
            _ => self.respond_to_take_main_action(),
        }
    }
}

fn parse_main_action(input: &str) -> PlayerAction {
    use PlayerAction::*;

    let cleaned_input = input.trim().to_lowercase();
    let cleaned_input = cleaned_input.as_str();

    if cleaned_input == "" { return NoAction; }

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
            println!("battle <card number> - Initiate a battle with a character from the board. 
                      Your leader is represented by 'L' instead of a number.");
            println!("end - End your turn.");
            NoAction
        },
        "end" => {
            End
        }
        _ => NoAction,
    }
}