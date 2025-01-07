#![allow(unused)]

use std::fmt;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc::{channel, Sender, Receiver};

use nom::{IResult, bytes::complete::tag};
use serde::{Serialize, Deserialize};

fn main() {
    let franky = Card {
        name: "Franky".to_string(),
        identifier: "OP09-072".to_string(),
        art: "P1".to_string(),
        cost: CardCost(4),
        category: CardCategory::Character,
        power: Some(CardPower(5000)),
        counter_power: Some(CounterPower(1000)),
        attribute: vec![Attribute::Strike],
        color: vec![CardColor::Purple],
        types: vec!["Straw Hat Crew".to_string()],
        effects: vec![Effect::Blocker, Effect::TimedEffect(Timing::OnPlay, EffectCost::MinusDon(2), Box::new(Effect::Draw(2)))],
        attached_don: vec![],
    };

    let basic_don = Card {
        name: "Your Turn +1000".to_string(),
        identifier: "DON-don".to_string(),
        art: "P0".to_string(),
        cost: CardCost(0),
        category: CardCategory::Don,
        power: None,
        counter_power: None,
        attribute: vec![],
        color: vec![],
        types: vec![],
        effects: vec![Effect::PlusPower(1000)],
        attached_don: vec![],
    };

    let franky_serial = serde_json::to_string_pretty(&franky).unwrap();
    
    let deck = vec![franky.clone(), franky.clone(), franky.clone(), franky.clone()];
    
    println!("{:?}", deck);

    let mut file = File::create("assets/card_data/OP09-072-P0.json").unwrap();
    file.write(franky_serial.as_bytes()).unwrap();

}

fn parse_deck_list(deck_list: &str) -> Result<(Card, Vec<Card>, Vec<Card>), DeckParseError> {
    let leader: Card;
    let mut main_deck: Vec<Card> = vec![];
    let mut don_deck: Vec<Card> = vec![];

    struct DeckListEntry {
        quantity: i32,
        id: String,
        name: String,
        art: String,
    }

    let mut deck_list_entries: Vec<DeckListEntry> = vec![];

    fn is_comment(line: &str) -> bool {
        line.starts_with("//")
    }

    for line in deck_list.lines() {
        if is_comment(line) {
            continue;
        }

        let mut art = String::new();

        let line_contents = line.split_whitespace().collect::<Vec<&str>>();
        match line_contents.len() {
            n if n < 3 => return Err(DeckParseError::IncompleteLine(line.to_string())),
            3 => art = "P0".to_string(),
            4 => art = line_contents[3].to_string(),
            n if n > 4 => return Err(DeckParseError::TooManyFields(line.to_string())),
            _ => unreachable!()
        };

        let quantity = line_contents[0].parse::<i32>().unwrap();
        let id = line_contents[1].to_string();
        let name = line_contents[2].to_string();

        let entry = DeckListEntry {
            quantity,
            id,
            name,
            art,
        };

        deck_list_entries.push(entry);
    }

    todo!();
}

pub enum DeckParseError {
    NoLeader,
    TooManyLeaders,
    TooManyCopies(i32), // Max number of any particular card id in the main deck is 4 copies.
    IncompleteLine(String),
    TooManyFields(String),
    InvalidCardId(String),
    InvalidCardName(String),
    InvalidCardArt(String),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CardColor {
    Red,
    Blue,
    Green,
    Purple,
    Black,
    Yellow,
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CardCost(i32);


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CardPower(i32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CounterPower(i32);


 
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CardCategory {
    Leader,
    Character,
    Event,
    Stage,
    Don,
}


#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Attribute {
    Slash, // SL
    Strike, // ST
    Ranged, // RN
    Special, // SP
    Wisdom, // WS
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub name: String,
    pub identifier: String, // Identifies the card outside of the card art, determines uniqueness in a deck list.
    pub art: String,
    pub cost: CardCost,
    pub category: CardCategory,
    pub power: Option<CardPower>, // Only Leader and Character cards have a power.
    pub counter_power: Option<CounterPower>, // Only Character cards have a counter power.
    pub attribute: Vec<Attribute>, // Only Leader and Character cards have a attribute, and can have multiple attributes.
    pub color: Vec<CardColor>, // Some cards have more than one color.
    pub types: Vec<String>, // Some cards have more than one type.
    pub effects: Vec<Effect>, 
    pub attached_don: Deck, // Only Leader and Character cards can have a don attached
}

pub type Deck = Vec<Card>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub leader: Card,
    pub main_deck: Deck,
    pub don_deck: Deck,
    pub hand: Deck,
    pub trash: Deck,
}

pub struct PlayerClient {
    pub player: Box<Player>,
    pub tx: Sender<PlayerAction>,
    pub rx: Receiver<ServerMessage>,
}

pub struct PlayField {
    pub player_1: Box<Player>,
    pub player_2: Box<Player>,
    pub p1_sender: Sender<ServerMessage>,
    pub p2_sender: Sender<ServerMessage>,
    pub p1_receiver: Receiver<PlayerAction>,
    pub p2_receiver: Receiver<PlayerAction>,
    pub p1_life_area: Deck,
    pub p2_life_area: Deck,
    pub p1_stage_area: Deck,
    pub p2_stage_area: Deck,
    pub p1_character_area: Deck,
    pub p2_character_area: Deck,
    pub p1_rested_character_area: Deck,
    pub p2_rested_character_area: Deck,
    pub p1_main_deck_count: i32,
    pub p2_main_deck_count: i32,
    pub p1_don_deck_count: i32,
    pub p2_don_deck_count: i32,
    pub p1_active_don_area: Deck,
    pub p2_active_don_area: Deck,
    pub p1_rested_don_area: Deck,
    pub p2_rested_don_area: Deck,
}

impl PlayField {
    pub fn setup(player_1: Box<Player>, player_2: Box<Player>) -> (PlayField, PlayerClient, PlayerClient) {
        let (p1_sender, p1_server_receiver) = channel();
        let (p2_sender, p2_server_receiver) = channel();

        let (p1_client_sender, p1_receiver) = channel();
        let (p2_client_sender, p2_receiver) = channel();

        (PlayField {
            player_1: player_1.clone(),
            player_2: player_2.clone(),
            p1_sender,
            p2_sender,
            p1_receiver,
            p2_receiver,
            p1_life_area: Deck::new(),
            p2_life_area: Deck::new(),
            p1_stage_area: Deck::new(),
            p2_stage_area: Deck::new(),
            p1_character_area: Deck::new(),
            p2_character_area: Deck::new(),
            p1_rested_character_area: Deck::new(),
            p2_rested_character_area: Deck::new(),
            p1_main_deck_count: 0,
            p2_main_deck_count: 0,
            p1_don_deck_count: 0,
            p2_don_deck_count: 0,
            p1_active_don_area: Deck::new(),
            p2_active_don_area: Deck::new(),
            p1_rested_don_area: Deck::new(),
            p2_rested_don_area: Deck::new(),
        },
        PlayerClient {
            player: player_1.clone(),
            tx: p1_client_sender,
            rx: p1_server_receiver,
        },
        PlayerClient {
            player: player_2.clone(),
            tx: p2_client_sender,
            rx: p2_server_receiver,
        })
    }
}

pub enum PlayerAction {}
pub enum ServerMessage {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TurnPhase {
    Refresh,
    Draw,
    Don,
    Main,
    End,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MainPhaseAction {
    PlayCard, 
    ActivateCardEffect,
    AttachDon,
    Battle, 
}

pub const MAX_CHARACTER_AREA: i32 = 5;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Timing {
    OnPlay,
    WhenAttacking,
    ActivateMain,
    CounterPhase,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effect {
    Blocker,
    Draw(i32),
    OncePerTurn,
    PlusPower(i32),
    Rush,
    TimedEffect(Timing, EffectCost, Box<Effect>),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectCost {
    MinusDon(i32),
    RestDon(i32),
    DonAttached(Box<Card>, i32),
    Zero, // Needed for timed effects that don't require a cost, makes more sense than doing `Option<EffectCost>` everywhere.
}

mod display_implementations {
    use super::*;

    impl fmt::Display for CardColor {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use CardColor::*;
            let mut val = String::new();
            match self {
                Red => val = "R".into(),
                Blue => val = "B".into(),
                Green => val = "G".into(),
                Purple => val = "P".into(),
                Black => val = "K".into(),
                Yellow => val = "Y".into(),
            }
            write!(f, "{val}")?;
            Ok(())
        }
    }

    impl fmt::Display for CardCost {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)?;
            Ok(())
        }
    }

    impl fmt::Display for CardPower {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)?;
            Ok(())
        }
    }

    impl fmt::Display for CounterPower {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)?;
            Ok(())
        }
    }

    impl fmt::Display for CardCategory {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use CardCategory::*;
            let mut val = String::new();
            match self {
                Leader => val = "Leader".into(),
                Character => val = "Character".into(),
                Event => val = "Event".into(),
                Stage => val = "Stage".into(),
                Don => val = "DON!! CARD".into(),
            }
            Ok(())
        }
    }

    impl fmt::Display for Attribute {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match *self {
                Attribute::Slash => write!(f, "SL")?,
                Attribute::Strike => write!(f, "ST")?,
                Attribute::Ranged => write!(f, "RN")?,
                Attribute::Special => write!(f, "SP")?,
                Attribute::Wisdom => write!(f, "WS")?,
            };
            Ok(())
        }
    }

    impl fmt::Display for Timing {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use Timing::*;
            let mut val: String = String::new();
            match self {
                OnPlay => val = "On Play".into(),
                WhenAttacking => val = "When Attacking".into(),
                ActivateMain => val = "Activate:Main".into(),
                CounterPhase => val = "Counter".into(),
            }
            write!(f, "{val}")?;
            Ok(())
        }
    }

    impl fmt::Display for EffectCost {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use EffectCost::*;
            match self {
                MinusDon(n) => write!(f, "DON!! -{}", n)?,
                RestDon(n) => write!(f, "{n}")?,
                DonAttached(_, n) => write!(f, "DON!!x{n}")?,
                Zero => write!(f, "")?,
            }

            Ok(())
        }
    }

    impl fmt::Display for Effect {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use Effect::*;
            match self {
                Blocker => write!(f, "Blocker")?,
                TimedEffect(timing, cost, effect) => write!(f, "{} {} {}", timing, cost, effect)?,
                Draw(i) => write!(f, "Draw {}", i)?,
                OncePerTurn => write!(f, "Once Per Turn")?,
                Rush => write!(f, "Rush")?,
                PlusPower(i) => write!(f, "+{i}")?,
            }
            Ok(())
        }
    }

    impl fmt::Display for Card {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "--------------------------------------\n")?;
            write!(f, "| {}                          {} ", self.cost, self.power.unwrap_or(CardPower(0)))?; 
            for att in self.attribute.iter() { 
                write!(f, "{}", att)?;
            } 
            write!(f, "  |\n")?;
            write!(f, "|                                    |\n")?;
            write!(f, "|                                    |\n")?;
            write!(f, "|                                    |\n")?;
            write!(f, "|                                    |\n")?;
            write!(f, "|  {}                              |\n", self.counter_power.unwrap_or(CounterPower(0)))?;
            write!(f, "|                                    |\n")?;
            for effect in self.effects.iter() {
                write!(f, "|  {}  |\n", effect)?;
            }
            for i in 0..(5 - self.effects.len()) {
                write!(f, "|                                    |\n")?;
            }
            write!(f, "|               {}               |\n", self.category)?;
            write!(f, "|               {}               |\n", self.name)?;
            write!(f, "| ")?; 
            for c in self.color.iter() {
                write!(f, "{} ", c)?;
            }
            write!(f, "  ")?;
            for t in self.types.iter() {
                write!(f, " {} ", t)?;
            } 
            write!(f, "        |\n")?;
            write!(f, "---------------------------------------\n")?;

            Ok(())
        }
    }
}