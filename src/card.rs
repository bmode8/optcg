use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use super::game::*;

pub fn install_card_data() {
    #![allow(non_snake_case)]
    use Attribute::*;
    use CardCategory::*;
    use CardColor::*;
    use Effect::*;
    use EffectCost::*;
    use Facing::*;
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
        facing: FaceDown,
    };

    let ST01_001 = Card::new(
        "Monkey D. Luffy".to_string(),
        "ST01-001".to_string(),
        "P0".to_string(),
        CardCost(0),
        Leader(5),
        Some(CardPower(5000)),
        None,
        vec![Strike],
        vec![Red],
        vec!["Supernovas".to_string(), "Straw Hat Crew".to_string()],
        vec![TimedEffect(
            ActivateMain,
            Zero,
            vec![OncePerTurn, GiveRestedDon(1)],
        )],
        FaceDown,
    );

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
        vec![
            TimedEffect(
                WhenAttacking,
                DonAttached(1),
                vec![OpponentNoBlocker(Condition::PowerAndAbove(5000))],
            ),
            TimedEffect(Trigger, Zero, vec![PlayCard]),
        ],
        FaceDown,
    );

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
        vec![],
        FaceDown,
    );

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
        vec![TimedEffect(DuringTurn, DonAttached(2), vec![Rush])],
        FaceDown
    );

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
        vec![TimedEffect(
            WhenAttacking,
            DonAttached(1),
            vec![GiveOtherCardPower(1000)],
        )],
        FaceDown
    );

    let ST01_006 = Card::new(
        "Tony Tony Chopper".into(),
        "ST01-006".into(),
        "P0".into(),
        CardCost(1),
        Character,
        Some(CardPower(1000)),
        None,
        vec![Strike],
        vec![Red],
        vec!["Animal".into(), "Straw Hat Crew".into()], 
        vec![Blocker],
        FaceDown,
    );

    let ST01_007 = Card::new(
        "Nami".into(),
        "ST01-007".into(),
        "P0".into(),
        CardCost(1),
        Character,
        Some(CardPower(1000)),
        Some(CounterPower(1000)),
        vec![Special],
        vec![Red],
        vec!["Straw Hat Crew".into()],
        vec![TimedEffect(ActivateMain, Zero, vec![OncePerTurn, GiveRestedDon(1)])],
        FaceDown,
    );

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
        vec![],
        FaceDown,
    );

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
        vec![],
        FaceDown,
    );

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
        vec![],
        FaceDown,
    );

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
        vec![TimedEffect(OnPlay, Zero, vec![GiveRestedDon(2)])],
        FaceDown,
    );

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
        vec![
            Rush,
            TimedEffect(
                WhenAttacking,
                DonAttached(2),
                vec![OpponentNoBlocker(Condition::None)],
            ),
        ],
        FaceDown
    );

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
        vec![TimedEffect(
            DuringTurn,
            DonAttached(1),
            vec![PlusPower(1000)],
        )],
        FaceDown
    );

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
        vec![
            TimedEffect(CounterPhase, Zero, vec![PlusPowerForBattle(3000)]),
            TimedEffect(Trigger, Zero, vec![PlusPower(1000)]),
        ],
        FaceDown
    );

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
        vec![
            TimedEffect(Main, Zero, vec![KnockOutWithPowerLessThan(6000)]),
            TimedEffect(Trigger, Zero, vec![KnockOutWithPowerLessThan(6000)]),
        ],
        FaceDown
    );

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
        effects: vec![
            Effect::Blocker,
            Effect::TimedEffect(
                Timing::OnPlay,
                EffectCost::MinusDon(2),
                vec![Effect::Draw(2)],
            ),
        ],
        attached_don: vec![],
        facing: FaceDown,
    };

    let current_cards = vec![
        DON_don, ST01_001, ST01_002, ST01_003, ST01_004, ST01_005, ST01_006, ST01_007, ST01_008, ST01_009, ST01_010,
        ST01_011, ST01_012, ST01_013, ST01_014, ST01_015, OP09_072,
    ];

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

pub fn parse_deck_list(deck_list: &str) -> Result<(Card, Deck, Deck), DeckError> {
    struct DeckListEntry {
        quantity: i32,
        id: String,
        art: String,
    }

    let mut deck_list_entries: Vec<DeckListEntry> = vec![];

    for line in deck_list.lines() {
        if is_comment(line) {
            continue;
        }

        let mut art = String::new();

        let line_contents = line.split_whitespace().collect::<Vec<&str>>();
        let line_len = line_contents.len();

        match line_len {
            n if n < 2 => return Err(DeckError::IncompleteLine(line.to_string())),
            2 => art = "P0".to_string(),
            3 => art = parse_art(line_contents[2])?,
            n if n > 3 => {
                if is_art(line_contents[line_len - 1]) {
                    art = parse_art(line_contents[line_len - 1])?;
                }
            }
            _ => unreachable!(),
        }

        let quantity = line_contents[0].parse::<i32>().unwrap();
        let id = line_contents[1].to_string();

        let entry = DeckListEntry {
            quantity,
            id,
            art,
        };

        deck_list_entries.push(entry);
    }

    let mut cards_used_in_deck: Vec<Card> = vec![];

    for entry in deck_list_entries.iter() {
        let filename: String;

        if entry.art == "" { 
            filename = format!("{}.json", entry.id); 
        } else { 
            filename = format!("{}-{}.json", entry.id, entry.art); 
        }

        let Ok(card_data) = std::fs::read_to_string(format!("assets/card_data/{}", filename))
        else {
            panic!("Could not find card data for {filename}");
        };

        let card: Card = serde_json::from_str(&card_data).unwrap();

        for _ in 0..entry.quantity {
            cards_used_in_deck.push(card.clone());
        }
    }

    validate_deck(&cards_used_in_deck)?;

    Ok((
        cards_used_in_deck.iter().find(|card| card.is_leader()).unwrap().clone(),
        cards_used_in_deck.iter().filter(|card| !card.is_don() && !card.is_leader()).map(|card| card.clone()).collect::<Vec<Card>>(),
        cards_used_in_deck.iter().filter(|card| card.is_don()).map(|card| card.clone()).collect::<Vec<Card>>(),
    ))
}

pub fn validate_deck(cards: &Vec<Card>) -> Result<(), DeckError> {
    if cards.len() != 61 {
        return Err(DeckError::InvalidDeckLength(cards.len()));
    }
    let mut leader_count = 0;
    cards.iter().for_each(|card| if card.is_leader() { leader_count += 1; });
    if leader_count > 1 {
        return Err(DeckError::TooManyLeaders);
    }
    
    let mut don_count = 0;
    let mut id_hash: HashMap<String, i32> = HashMap::new();

    for card in cards.iter() {
        if card.is_don() { 
            don_count += 1;
            continue; 
        }

        let current = id_hash.insert(card.identifier.clone(), 1);
        match current {
            None => (),
            Some(i) => { id_hash.entry(card.identifier.clone()).and_modify(|e| *e = i + 1); },
        }
    }

    if don_count != 10 {
        return Err(DeckError::NotEnoughDon);
    }

    for entry in id_hash.iter() {
        if *entry.1 > 4 {
            return Err(DeckError::TooManyCopies(entry.0.clone(), *entry.1));
        }
    }


    Ok(())
}

fn is_comment(line: &str) -> bool {
    line.starts_with("//")
}

fn is_art(art: &str) -> bool {
    if !art.starts_with("(") {
        false
    } else if !art.ends_with(")") {
        false
    } else {
        true
    }
}

fn parse_art(art: &str) -> Result<String, DeckError> {
    // Should be formatted as simply wrapped in parentheses.

    let art = String::from(art);
    let mut art_iter = art.chars();
    if let Some(first_char) = art_iter.next() {
        match first_char {
            '(' => (),
            _ => return Err(DeckError::ArtSyntaxError(art.to_string())),
        }
    } else {
        return Err(DeckError::ArtSyntaxError(art.to_string()));
    };

    if let Some(last_char) = art_iter.next_back() {
        match last_char {
            ')' => (),
            _ => return Err(DeckError::ArtSyntaxError(art.to_string())),
        }
    } else {
        return Err(DeckError::ArtSyntaxError(art.to_string()));
    };

    Ok(art_iter.collect())
}

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
    NotEnoughDon, // Requires exactly 10 DON!! cards.
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
            facing
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

    pub fn set_faceup(&mut self) {
        self.facing = Facing::FaceUp;
    }
}

pub type Deck = Vec<Card>;