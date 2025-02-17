use std::io::stdin;

use futures::prelude::*;
use log::*;
use serde_json::Value;
use simplelog::*;
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tokio_serde::formats::*;
use tokio_serde::Framed;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use optcg::game::*;
use optcg::player::*;
use optcg::{print_hand, PlayerAction, ServerMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    debug!("Connected to server.");

    let mut client = Client::new(
        Turn::P1, // this is a placeholder.
        Box::new(Player::empty()),
        Box::new(Player::empty()),
        Box::new(PublicPlayfieldState::empty()),
        &mut stream,
    );

    loop {
        client.handle_messages().await;
        client.send_action(PlayerAction::Idle).await;
    }
}

struct Client<'stream> {
    this_id: Turn,
    this_player: Box<Player>,
    other_player: Box<Player>,
    public_playfield_state: Box<PublicPlayfieldState>,
    writer: Framed<
        FramedWrite<WriteHalf<'stream>, LengthDelimitedCodec>,
        Value,
        Value,
        Json<Value, Value>,
    >,
    reader: Framed<
        FramedRead<ReadHalf<'stream>, LengthDelimitedCodec>,
        Value,
        Value,
        Json<Value, Value>,
    >,
}

impl<'stream> Client<'stream> {
    pub fn new(
        this_id: Turn,
        this_player: Box<Player>,
        other_player: Box<Player>,
        public_playfield_state: Box<PublicPlayfieldState>,
        socket: &'stream mut TcpStream,
    ) -> Self {
        let (rx, tx) = socket.split();

        let read_frame = FramedRead::new(rx, LengthDelimitedCodec::new());
        let reader =
            tokio_serde::SymmetricallyFramed::new(read_frame, SymmetricalJson::<Value>::default());
        let write_frame = FramedWrite::new(tx, LengthDelimitedCodec::new());
        let writer =
            tokio_serde::SymmetricallyFramed::new(write_frame, SymmetricalJson::<Value>::default());
        Self {
            this_id,
            this_player,
            other_player,
            public_playfield_state,
            writer,
            reader,
        }
    }

    pub async fn send_action(&mut self, action: PlayerAction) {
        self.writer
            .send(serde_json::from_str(serde_json::to_string(&action).unwrap().as_str()).unwrap())
            .await
            .unwrap();
    }

    pub async fn handle_messages(&mut self) {
        while let Some(next) = self.reader.try_next().await.unwrap() {
            let message = serde_json::from_value::<ServerMessage>(next).unwrap();
            match message {
                ServerMessage::Connected => {}
                ServerMessage::PlayerId(id) => {
                    self.this_id = id;
                }
                ServerMessage::RequestDeck => {}
                ServerMessage::QueryMulligan => {
                    print_hand(&self.this_player.hand);
                    return self.respond_to_query_mulligan().await;
                }
                ServerMessage::TakeMainAction => {
                    return self.respond_to_take_main_action().await;
                }
                ServerMessage::InsufficientDon => {
                    println!("Insufficient DON!! to play this card.");
                }
                ServerMessage::InvalidTarget => {
                    println!("Invalid target.");
                }
                ServerMessage::DiscardCharacter => {
                    return self.respond_to_discard_character().await;
                }
                ServerMessage::CannotPlayCounterEventDuringMainPhase => {
                    println!("Cannot play a counter event during the main phase.");
                }
                ServerMessage::NoTargetsMeetConditions => {
                    println!("No targets meet the conditions for this effect.");
                }
                ServerMessage::QueryTargetOpposingCharacter => {
                    return self.respond_to_query_target_opposing_character().await;
                }
                ServerMessage::QueryTargetSelfCharacterOrLeader => {
                    return self
                        .respond_to_query_target_self_character_or_leader()
                        .await;
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
            break;
        }
    }

    pub async fn respond_to_query_mulligan(&mut self) {
        println!("Mulligan? [y/N]  ");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        loop {
            match input.trim().to_lowercase().as_str() {
                "y" => return self.send_action(PlayerAction::TakeMulligan).await,
                "n" | "" => return self.send_action(PlayerAction::NoAction).await,
                _ => continue,
            }
        }
    }

    pub async fn respond_to_take_main_action(&mut self) {
        loop {
            println!("Hand: ");
            print_hand(&self.this_player.hand);

            println!("Action: ");
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            let main_action = parse_main_action(input.trim().to_lowercase().as_str());
            return self.send_action(main_action).await;
        }
    }

    pub async fn respond_to_query_target_opposing_character(&mut self) {
        println!("Select a Character to target:");
        let target_idx: usize;
        match self.this_id {
            Turn::P1 => {
                for (i, character) in self
                    .public_playfield_state
                    .p2_character_area
                    .iter()
                    .enumerate()
                {
                    println!("{i}: {}", character);
                }
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();

                target_idx = input.trim().parse::<usize>().unwrap();
            }
            Turn::P2 => {
                for (i, character) in self
                    .public_playfield_state
                    .p1_character_area
                    .iter()
                    .enumerate()
                {
                    println!("{i}: {}", character);
                }
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();

                target_idx = input.trim().parse::<usize>().unwrap();
            }
        }

        self.send_action(PlayerAction::TargetOpposingCharacter(target_idx))
            .await;
    }

    pub async fn respond_to_query_target_self_character_or_leader(&mut self) {
        println!("Select a Character to target:");
        let target_idx: char;
        match self.this_id {
            Turn::P1 => {
                for (i, character) in self
                    .public_playfield_state
                    .p1_character_area
                    .iter()
                    .enumerate()
                {
                    println!("{i}: {}", character);
                }
                println!("L: Leader {}", self.this_player.leader);
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();

                target_idx = input.trim().to_lowercase().parse::<char>().unwrap();
            }
            Turn::P2 => {
                for (i, character) in self
                    .public_playfield_state
                    .p2_character_area
                    .iter()
                    .enumerate()
                {
                    println!("{i}: {}", character);
                }
                println!("L: Leader {}", self.this_player.leader);
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();

                target_idx = input.trim().to_lowercase().parse::<char>().unwrap();
            }
        }

        self.send_action(PlayerAction::TargetSelfCharacterOrLeader(target_idx))
            .await;
    }

    pub async fn respond_to_discard_character(&mut self) {
        println!("Discard which character?");
        match self.this_id {
            Turn::P1 => loop {
                for (i, character) in self
                    .public_playfield_state
                    .p1_character_area
                    .iter()
                    .enumerate()
                {
                    println!("{i}: {}", character);
                }
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();

                let target_idx = input.trim().parse::<usize>();
                if target_idx.is_err() {
                    continue;
                }
                let target_idx = target_idx.unwrap();
                if target_idx > self.public_playfield_state.p1_character_area.len() - 1 {
                    println!("Invalid target.");
                    continue;
                }
                self.send_action(PlayerAction::DiscardCharacter(target_idx))
                    .await;
                return;
            },
            Turn::P2 => loop {
                for (i, character) in self
                    .public_playfield_state
                    .p2_character_area
                    .iter()
                    .enumerate()
                {
                    println!("{i}: {}", character);
                }
                let mut input = String::new();
                stdin().read_line(&mut input).unwrap();

                let target_idx = input.trim().parse::<usize>();
                if target_idx.is_err() {
                    continue;
                }
                let target_idx = target_idx.unwrap();
                if target_idx > self.public_playfield_state.p1_character_area.len() - 1 {
                    println!("Invalid target.");
                    continue;
                }
                self.send_action(PlayerAction::DiscardCharacter(target_idx))
                    .await;

                return;
            },
        }
    }
}

fn parse_main_action(input: &str) -> PlayerAction {
    use PlayerAction::*;

    debug!("Parsing Main Action");
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
            println!("activate <card number, 'L', or 'S'> - Activate a card effect on the board.");
            println!("attach <card number or 'L'> - Attach a DON!! card from the active DON!! area to your leader or a character in play.");
            println!("battle <card number or 'L'> - Initiate a battle with your leader or an active character in play.");
            println!("end - End your turn.");
            println!();
            println!("Press enter to continue...");
            let mut _temp = String::new();
            stdin().read_line(&mut _temp).unwrap();
            NoAction
        }
        "end" => End,
        "play" => {
            if words.len() < 2 {
                return NoAction;
            }
            let card_number = words[1].parse::<usize>().unwrap();
            MainPlayCard(card_number)
        }
        "activate" => {
            if words.len() < 2 {
                return NoAction;
            }
            let card_id = words[1].parse::<char>().unwrap();
            MainActivateCardEffect(card_id)
        }
        _ => NoAction,
    }
}
