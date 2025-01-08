#![allow(unused)]

use std::fmt;
use std::fs::File;
use std::io::Write;
use std::sync::mpsc::{channel, Sender, Receiver};

use serde::{Serialize, Deserialize};
use serde_json::map::Entry;

fn main() {

    install_card_data();
}

fn install_card_data() {
    #![allow(non_snake_case)]
    use CardCategory::*;
    use CardColor::*;
    use Attribute::*;
    use Effect::*;
    use EffectCost::*;
    use Timing::*;
    
    let DON_don = Card {
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

    let ST01_001 = Card::new(
        "Monkey D. Luffy".to_string(),
        "ST01-001".to_string(), 
        "P0".to_string(), 
        CardCost(0), 
        Leader, 
        Some(CardPower(5000)), 
        None, 
        vec![Strike], 
        vec![Red], 
        vec!["Supernovas".to_string(), "Straw Hat Crew".to_string()],
        vec![TimedEffect(ActivateMain, Zero, vec![OncePerTurn, GiveRestedDon(1)])]);
    
    let ST01_002 = Card::new(
        "Usopp".to_string(),
        "ST01-002".to_string(), 
        "P0".to_string(), 
        CardCost(2), 
        Character, 
        Some(CardPower(2000)), 
        Some(CounterPower(1000)), 
        vec![Ranged], 
        vec![Red], 
        vec!["Straw Hat Crew".to_string()],
        vec![TimedEffect(WhenAttacking, DonAttached(1), vec![OpponentNoBlocker(Condition::PowerAndAbove(5000))]),
             TimedEffect(Trigger, Zero, vec![PlayCard])]);
    
    let ST01_003 = Card::new(   
        "Carue".to_string(),
        "ST01-003".to_string(), 
        "P0".to_string(), 
        CardCost(1), 
        Character, 
        Some(CardPower(3000)), 
        Some(CounterPower(1000)), 
        vec![Strike], 
        vec![Red], 
        vec!["Animal".to_string(), "Alabasta".to_string()],
        vec![]);

    let ST01_004 = Card::new(
        "Sanji".to_string(),
        "ST01-004".to_string(), 
        "P0".to_string(), 
        CardCost(2), 
        Character, 
        Some(CardPower(4000)), 
        None, 
        vec![Strike], 
        vec![Red], 
        vec!["Straw Hat Crew".to_string()],
        vec![TimedEffect(DuringTurn, DonAttached(2), vec![Rush])]);
    
    let ST01_005 = Card::new(
        "Jinbe".to_string(),
        "ST01-005".to_string(), 
        "P0".to_string(), 
        CardCost(3), 
        Character, 
        Some(CardPower(5000)), 
        None, 
        vec![Strike], 
        vec![Red], 
        vec!["Fish-Man".to_string(), "Straw Hat Crew".to_string()],
        vec![TimedEffect(WhenAttacking, DonAttached(1), vec![GiveOtherCardPower(1000)])]);

    let ST01_008 = Card::new(
        "Nico Robin".to_string(),
        "ST01-008".to_string(), 
        "P0".to_string(), 
        CardCost(3), 
        Character, 
        Some(CardPower(5000)), 
        Some(CounterPower(1000)), 
        vec![Wisdom], 
        vec![Red], 
        vec!["Straw Hat Crew".to_string()],
        vec![]);

    let ST01_009 = Card::new(
        "Nefertari Vivi".to_string(),
        "ST01-009".to_string(), 
        "P0".to_string(), 
        CardCost(2), 
        Character, 
        Some(CardPower(4000)), 
        Some(CounterPower(1000)), 
        vec![Slash], 
        vec![Red], 
        vec!["Alabasta".to_string()],
        vec![]);

    let ST01_010 = Card::new(
        "Franky".to_string(),
        "ST01-010".to_string(), 
        "P0".to_string(), 
        CardCost(4), 
        Character, 
        Some(CardPower(6000)), 
        Some(CounterPower(1000)), 
        vec![Strike], 
        vec![Red], 
        vec!["Straw Hat Crew".to_string()],
        vec![]);

    let ST01_011 = Card::new(
        "Brook".to_string(),
        "ST01-011".to_string(), 
        "P0".to_string(), 
        CardCost(2), 
        Character, 
        Some(CardPower(3000)), 
        Some(CounterPower(2000)), 
        vec![Slash], 
        vec![Red], 
        vec!["Straw Hat Crew".to_string()],
        vec![TimedEffect(OnPlay, Zero, vec![GiveRestedDon(2)])]);

    let ST01_012 = Card::new(
        "Monkey D. Luffy".to_string(),
        "ST01-012".to_string(), 
        "P0".to_string(), 
        CardCost(5), 
        Character, 
        Some(CardPower(6000)), 
        None, 
        vec![Strike], 
        vec![Red], 
        vec!["Supernovas".to_string(), "Straw Hat Crew".to_string()],
        vec![Rush, TimedEffect(WhenAttacking, DonAttached(2), vec![OpponentNoBlocker(Condition::None)])]);
    
    let ST01_013 = Card::new(
        "Roronoa Zoro".to_string(), 
        "ST01-013".to_string(), 
        "P0".to_string(), 
        CardCost(3), 
        Character, 
        Some(CardPower(5000)), 
        None, 
        vec![Slash], 
        vec![Red], 
        vec!["Supernovas".to_string(), "Straw Hat Crew".to_string()],
        vec![TimedEffect(DuringTurn, DonAttached(1), vec![PlusPower(1000)])]);

    let ST01_014 = Card::new(
        "Guard Point".to_string(), 
        "ST01-014".to_string(), 
        "P0".to_string(), 
        CardCost(1), 
        Event, 
        None, 
        None, 
        vec![], 
        vec![Red], 
        vec!["Animal".to_string(), "Straw Hat Crew".to_string()],
        vec![TimedEffect(CounterPhase, Zero, vec![PlusPowerForBattle(3000)]),
             TimedEffect(Trigger, Zero, vec![PlusPower(1000)])]);

    let ST01_015 = Card::new(
        "Jet Pistol".to_string(), 
        "ST01-015".to_string(), 
        "P0".to_string(), 
        CardCost(4), 
        Event, 
        None, 
        None, 
        vec![], 
        vec![Red], 
        vec!["Supernovas".to_string(), "Straw Hat Crew".to_string()],
        vec![TimedEffect(Main, Zero, vec![KnockOutWithPowerLessThan(6000)]),
             TimedEffect(Trigger, Zero, vec![KnockOutWithPowerLessThan(6000)])]);
    
    let OP09_072 = Card {
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
        effects: vec![Effect::Blocker, Effect::TimedEffect(Timing::OnPlay, EffectCost::MinusDon(2), vec![Effect::Draw(2)])],
        attached_don: vec![],
    };

    let current_cards = vec![DON_don, ST01_001, ST01_002, ST01_003, ST01_004, ST01_005, ST01_008, 
        ST01_009, ST01_010, ST01_011, ST01_012, ST01_013, ST01_014, ST01_015, OP09_072];
    
    for card in current_cards {
        match card.category {
            CardCategory::Don => { 
                let filename = format!("{}.json", card.identifier);
                let card_data = serde_json::to_string_pretty(&card).unwrap();
                std::fs::write(format!("assets/card_data/{}", filename), card_data).unwrap();
            }
            _ => {
                let filename = format!("{}-{}.json", card.identifier, card.art);
                let card_data = serde_json::to_string_pretty(&card).unwrap();
                std::fs::write(format!("assets/card_data/{}", filename), card_data).unwrap();
            }
        }
    }
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

    for line in deck_list.lines() {
        if is_comment(line) {
            continue;
        }

        let mut art = String::new();

        let line_contents = line.split_whitespace().collect::<Vec<&str>>();
        match line_contents.len() {
            n if n < 3 => return Err(DeckParseError::IncompleteLine(line.to_string())),
            3 => art = "P0".to_string(),
            4 => art = parse_art(line_contents[3])?,
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

    let mut cards_used_in_deck: Vec<Card> = vec![];

    for entry in deck_list_entries.iter() {
        let filename = format!("{}-{}.json", entry.id, entry.art);
        let card_data = std::fs::read_to_string(format!("assets/card_data/{}", filename)).unwrap();
        let card: Card = serde_json::from_str(&card_data).unwrap();
        cards_used_in_deck.push(card);
    }

    todo!();
}

fn is_comment(line: &str) -> bool {
    line.starts_with("//")
}

fn parse_art(art: &str) -> Result<String, DeckParseError> {
    // Should be formatted as simply wrapped in parentheses.

    let mut art = String::from(art);
    let mut art_iter = art.chars();
    if let Some(first_char) = art_iter.next() {
        match first_char {
            '(' => (),
            _ => return Err(DeckParseError::ArtSyntaxError(art.to_string())),
        }
    } else {
        return Err(DeckParseError::ArtSyntaxError(art.to_string()));
    };

    if let Some(last_char) = art_iter.next_back() {
        match last_char {
            ')' => (),
            _ => return Err(DeckParseError::ArtSyntaxError(art.to_string())),
        }
    } else {
        return Err(DeckParseError::ArtSyntaxError(art.to_string()));
    };

    Ok(art_iter.collect())
}

pub enum DeckParseError {
    ArtSyntaxError(String),
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

impl Card {
    pub fn new(name: String, identifier: String, art: String, cost: CardCost, category: CardCategory, power: Option<CardPower>, counter_power: Option<CounterPower>, attribute: Vec<Attribute>, color: Vec<CardColor>, types: Vec<String>, effects: Vec<Effect>) -> Card {
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
            attached_don: vec![]
        }
    }
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
    Main, // basically ActivateMain, but for event cards.
    CounterPhase,
    DuringTurn,
    Trigger,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effect {
    Blocker,
    Draw(i32),
    GiveOtherCardPower(i32),
    GiveRestedDon(i32),
    KnockOutWithPowerLessThan(i32),
    OncePerTurn,
    OpponentNoBlocker(Condition),
    PlayCard,
    PlusPower(i32),
    PlusPowerForBattle(i32),
    Rush,
    TimedEffect(Timing, EffectCost, Vec<Effect>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    None,
    PowerAndAbove(i32),
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EffectCost {
    MinusDon(i32),
    RestDon(i32),
    DonAttached(i32),
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
                Main => val = "Main".into(),
                CounterPhase => val = "Counter".into(),
                DuringTurn => val = "".into(),
                Trigger => val = "Trigger".into(),
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
                DonAttached(n) => write!(f, "DON!!x{n}")?,
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
                TimedEffect(timing, cost, effect) => write!(f, "{} {} {:#?}", timing, cost, effect)?,
                Draw(i) => write!(f, "Draw {}", i)?,
                GiveOtherCardPower(i) => write!(f, "Give your Leader or 1 of your Characters other than this card +{i} power during this turn.")?,
                GiveRestedDon(i) => write!(f, "Give this Leader or 1 of your Characters {i} rested DON!! card(s).")?,
                KnockOutWithPowerLessThan(i) => write!(f, "K.O. 1 of your opponent's Characters with a power of {i} or less.")?,
                OncePerTurn => write!(f, "Once Per Turn")?,
                OpponentNoBlocker(condition) => write!(f, "Your opponent cannot activate Blocker of {:?} during this battle.", condition)?,
                Rush => write!(f, "Rush")?,
                PlayCard => write!(f, "Play this card.")?,
                PlusPower(i) => write!(f, "+{i}")?,
                PlusPowerForBattle(i) => write!(f, "Your Leader or 1 of your Characters gains +{i} for this battle.")?,
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