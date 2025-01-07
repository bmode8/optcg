#![allow(unused)]

use std::fmt;

fn main() {
    let franky = Card {
        name: "Franky".to_string(),
        identifier: "OP09-072".to_string(),
        art: "alt".to_string(),
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
    
    println!("{}", franky);
}

#[derive(Debug, Clone, Copy)]
pub enum CardColor {
    Red,
    Blue,
    Green,
    Purple,
    Black,
    Yellow,
}

impl fmt::Display for CardColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use CardColor::*;
        let mut val = String::new();
        match self {
            Red => val = "Red".into(),
            Blue => val = "Blue".into(),
            Green => val = "Green".into(),
            Purple => val = "Purple".into(),
            Black => val = "Black".into(),
            Yellow => val = "Yellow".into(),
        }
        write!(f, "{val}")?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CardCost(i32);

impl fmt::Display for CardCost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CardPower(i32);

impl fmt::Display for CardPower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CounterPower(i32);

impl fmt::Display for CounterPower {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)?;
        Ok(())
    }
}

 
#[derive(Debug, Clone, Copy)]
pub enum CardCategory {
    Leader,
    Character,
    Event,
    Stage,
    Don,
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

#[derive(Debug, Clone, Copy)]
pub enum Attribute {
    Slash, // SL
    Strike, // ST
    Ranged, // RN
    Special, // SP
    Wisdom, // WS
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
#[derive(Debug, Clone)]
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

pub type Deck = Vec<Card>;

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    pub leader: Card,
    pub main_deck: Deck,
    pub don_deck: Deck,
    pub hand: Deck,
    pub trash: Deck,
}

pub struct PlayField {
    pub player_1: Player,
    pub player_2: Player,
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
    pub fn setup(player_1: Player, player_2: Player) -> PlayField {
        PlayField {
            player_1,
            player_2,
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
        }
    }
}

#[derive(Debug, Clone)]
pub enum TurnPhase {
    Refresh,
    Draw,
    Don,
    Main,
    End,
}

#[derive(Debug, Clone)]
pub enum MainPhaseAction {
    PlayCard, 
    ActivateCardEffect,
    AttachDon,
    Battle, 
}

pub const MAX_CHARACTER_AREA: i32 = 5;

#[derive(Debug, Clone)]
pub enum Timing {
    OnPlay,
    WhenAttacking,
    ActivateMain,
    CounterPhase,
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

#[derive(Debug, Clone)]
pub enum Effect {
    Blocker,
    TimedEffect(Timing, EffectCost, Box<Effect>),
    Draw(i32),
    OncePerTurn,
    Rush,
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
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum EffectCost {
    MinusDon(i32),
    RestDon(i32),
    DonAttached(Box<Card>, i32),
    Zero, // Needed for timed effects that don't require a cost, makes more sense than doing `Option<EffectCost>` everywhere.
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