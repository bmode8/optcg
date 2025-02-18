use serde::{Deserialize, Serialize};

use super::game::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeckError {
    ArtSyntaxError(String),
    NoLeader,
    TooManyLeaders,
    TooManyCopies(String, i32), // Max number of any particular card id in the main deck is 4 copies.
    IncompleteLine(String),
    TooManyFields(String),
    InvalidCardId(String),
    InvalidCardName(String),
    InvalidCardArt(String),
    InvalidDeckLength(usize), // Requires exactly 61 cards total (Leader, Main deck, and DON!! deck)
    NotEnoughDon,             // Requires exactly 10 DON!! cards.
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
pub struct CardCost(pub i32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CardPower(pub i32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CounterPower(pub i32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CardCategory {
    Leader(i32), // Leader cards have a unique life total that cannot be determined from other data.
    Character,
    Event,
    Stage,
    Don,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Attribute {
    Slash,   // SL
    Strike,  // ST
    Ranged,  // RN
    Special, // SP
    Wisdom,  // WS
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Facing {
    FaceUp,
    FaceDown,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize,)]
pub enum Status {
    PowerPlus(i32),
    CostMinus(i32),
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
    pub color: Vec<CardColor>,     // Some cards have more than one color.
    pub types: Vec<String>,        // Some cards have more than one type.
    pub effects: Vec<Effect>,
    pub attached_don: Deck, // Only Leader and Character cards can have a don attached
    pub status: Vec<Status>,
    pub facing: Facing,
}

impl Card {
    pub fn new(
        name: String,
        identifier: String,
        art: String,
        cost: CardCost,
        category: CardCategory,
        power: Option<CardPower>,
        counter_power: Option<CounterPower>,
        attribute: Vec<Attribute>,
        color: Vec<CardColor>,
        types: Vec<String>,
        effects: Vec<Effect>,
        facing: Facing,
    ) -> Card {
        Card {
            name,
            identifier,
            art,
            cost,
            category,
            power,
            counter_power,
            attribute,
            color,
            types,
            effects,
            attached_don: vec![],
            status: vec![],
            facing,
        }
    }

    pub fn is_leader(&self) -> bool {
        match self.category {
            CardCategory::Leader(_) => true,
            _ => false,
        }
    }

    pub fn life(&self) -> i32 {
        match self.category {
            CardCategory::Leader(n) => n,
            _ => 0,
        }
    }

    pub fn is_don(&self) -> bool {
        match self.category {
            CardCategory::Don => true,
            _ => false,
        }
    }

    pub fn set_faceup(mut self) -> Self {
        self.facing = Facing::FaceUp;
        self
    }

    pub fn set_facedown(mut self) -> Self {
        self.facing = Facing::FaceDown;
        self
    }
}

pub type Deck = Vec<Card>;

pub trait SetFacing {
    fn set_faceup(self) -> Self;
    fn set_facedown(self) -> Self;
}

impl SetFacing for Deck {
    fn set_faceup(self) -> Deck {
        let mut set = Deck::new();
        for card in self.into_iter() {
            set.push(card.set_faceup());
        }

        set
    }

    fn set_facedown(self) -> Deck {
        let mut set = Deck::new();
        for card in self.into_iter() {
            set.push(card.set_facedown());
        }

        set
    }
}
