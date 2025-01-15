#![allow(unused)]
use std::collections::HashMap;
use std::fmt;
use std::io::{stdin, Read, Write};
use std::sync::mpsc::{channel, Receiver, Sender};

use log::*;
use rand::prelude::*;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

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
pub struct CardCost(i32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CardPower(i32);

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct CounterPower(i32);

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub leader: Card,
    pub main_deck: Deck,
    pub don_deck: Deck,
    pub hand: Deck,
    pub trash: Deck,
}

impl Player {
    pub fn draw(&mut self, n: i32) -> Result<(), ()>{
        for _ in 0..n {
            let drawn_card = self.main_deck.pop();

            if drawn_card.is_none() {
                error!("COULD NOT DRAW CARD");
                return Err(());
            }
            let drawn_card = drawn_card.unwrap();
            debug!("DRAWN CARD: {}", drawn_card);
            self.hand.push(drawn_card);
        }

        Ok(())
    }

    pub fn draw_don(&mut self, n: i32) -> Deck {
        let mut drawn_don = Deck::new();
        for _ in 0..n {
            let drawn_card = self.don_deck.pop();

            if drawn_card.is_none() {
                return drawn_don;
            }

            drawn_don.push(drawn_card.unwrap());
        }

        drawn_don
    }

    pub fn draw_out(&mut self, n: i32) -> Result<Deck, ()> {
        let mut drawn_out = Deck::new();
        for _ in 0..n {
            let drawn_card = self.main_deck.pop();

            if drawn_card.is_none() {
                return Err(());
            }

            drawn_out.push(drawn_card.unwrap());
        }

        Ok(drawn_out)
    }

    pub fn topdeck_hand(&mut self) {
        self.main_deck.append(&mut self.hand);
    }

    pub fn shuffle(&mut self, rng: &mut ThreadRng) {
        self.main_deck.shuffle(rng);
    }

    pub fn turn_hand_faceup(&mut self) {
        for card in self.hand.iter_mut() {
            card.set_faceup();
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerAction {
    TakeMulligan,
    NoAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    QueryMulligan,
    TakeMainAction,
    PlayerDataPayload(Box<Player>)
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
        }; 
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
            "end" => { self.tx.send(PlayerAction::NoAction).unwrap(); }
            _ => self.respond_to_take_main_action(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Turn {
    P1,
    P2,
}

pub type PlayerId = Turn;

pub struct PlayField {
    pub turn: Turn,
    pub turn_phase: TurnPhase,
    pub turn_n: i32,
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
    pub p1_active_don_area: Deck,
    pub p2_active_don_area: Deck,
    pub p1_rested_don_area: Deck,
    pub p2_rested_don_area: Deck,
    pub rng: ThreadRng,
}

impl PlayField {
    pub fn setup(
        mut player_1: Player,
        mut player_2: Player,
    ) -> (PlayField, MockPlayerClient, MockPlayerClient) {
        let (p1_sender, p1_server_receiver) = channel();
        let (p2_sender, p2_server_receiver) = channel();

        let (p1_client_sender, p1_receiver) = channel();
        let (p2_client_sender, p2_receiver) = channel();

        let mut rng = thread_rng();
        
        player_1.shuffle(&mut rng);
        player_2.shuffle(&mut rng);

        player_1.draw(5).unwrap();
        player_2.draw(5).unwrap();

        let mut player_1 = Box::new(player_1);
        let mut player_2 = Box::new(player_2);
        

        let mut player_1_client = MockPlayerClient {
            player: player_1.clone(),
            tx: p1_client_sender,
            rx: p1_server_receiver,
        };

        let mut player_2_client = MockPlayerClient {
            player: player_2.clone(),
            tx: p2_client_sender,
            rx: p2_server_receiver,
        };

        p1_sender.send(ServerMessage::QueryMulligan).unwrap();
        player_1_client.handle_message();
        let p1_mulligan = p1_receiver.recv().unwrap();

        if let PlayerAction::TakeMulligan = p1_mulligan {
            player_1.topdeck_hand();
            player_1.shuffle(&mut rng);
            player_1.draw(5).unwrap();  
            p1_sender.send(ServerMessage::PlayerDataPayload(player_1.clone())).unwrap();
            player_1_client.handle_message();
        }

        p2_sender.send(ServerMessage::QueryMulligan).unwrap();
        player_2_client.handle_message();
        let p2_mulligan = p2_receiver.recv().unwrap();

        if let PlayerAction::TakeMulligan = p2_mulligan {
            player_2.topdeck_hand();
            player_2.shuffle(&mut rng);
            player_2.draw(5).unwrap();
            p2_sender.send(ServerMessage::PlayerDataPayload(player_2.clone())).unwrap();
            player_2_client.handle_message();
        }

        let p1_life = player_1.draw_out(player_1.leader.life()).unwrap();
        let p2_life = player_2.draw_out(player_2.leader.life()).unwrap();
        
        // Begin turn 1.
        let p1_don = player_1.draw_don(1);

        (
            PlayField {
                turn: Turn::P1,
                turn_phase: TurnPhase::Main,
                turn_n: 1,
                player_1,
                player_2,
                p1_sender,
                p2_sender,
                p1_receiver,
                p2_receiver,
                p1_life_area: p1_life,
                p2_life_area: p2_life,
                p1_stage_area: Deck::new(),
                p2_stage_area: Deck::new(),
                p1_character_area: Deck::new(),
                p2_character_area: Deck::new(),
                p1_rested_character_area: Deck::new(),
                p2_rested_character_area: Deck::new(),
                p1_active_don_area: p1_don,
                p2_active_don_area: Deck::new(),
                p1_rested_don_area: Deck::new(),
                p2_rested_don_area: Deck::new(),
                rng,
            },
            player_1_client,
            player_2_client,
        )
    }

    pub fn check_loser(&self) -> Option<PlayerId> {
        // Return `None` if there is no loser, otherwise
        // return which of `P1` or `P2` lost.
        let p1_deck_len = self.player_1.main_deck.len();
        let p2_deck_len = self.player_2.main_deck.len();

        if p1_deck_len == 0 && p2_deck_len == 0 {
            panic!("Tie!??");
        }

        if p1_deck_len == 0 {
            return Some(PlayerId::P1)
        } else if p2_deck_len == 0 {
            return Some(PlayerId::P2)
        }

        None
    }

    pub fn trigger_loser(&self) -> Option<PlayerId> {
        todo!()
    }

    pub fn step(&mut self) {
        use TurnPhase::*;
        
        let current_player = match self.turn {
            Turn::P1 => &mut self.player_1,
            Turn::P2 => &mut self.player_2,
        };

        let (comms_tx, comms_rx) = match self.turn {
            Turn::P1 => (&mut self.p1_sender, &mut self.p1_receiver),
            Turn::P2 => (&mut self.p2_sender, &mut self.p2_receiver),
        };

        // Behold! A state machine!
        match self.turn_phase {
            Refresh => {
                debug!("(TURN) [REFRESH]");
                self.p1_active_don_area.append(&mut self.p1_rested_don_area);
                self.p1_character_area.append(&mut self.p1_rested_character_area);
                for card in self.p1_character_area.iter_mut() {
                    self.p1_active_don_area.append(&mut card.attached_don);
                }

                self.p2_active_don_area.append(&mut self.p2_rested_don_area);
                self.p2_character_area.append(&mut self.p2_rested_character_area);
                for card in self.p2_character_area.iter_mut() {
                    self.p2_active_don_area.append(&mut card.attached_don);
                }

                self.turn_phase = Draw;
            },
            Draw => {
                debug!("(TURN) [DRAW]");
                let res = current_player.draw(1);
                match res {
                    Ok(()) => { },
                    Err(()) => { return; },
                }
                comms_tx.send(ServerMessage::PlayerDataPayload(current_player.clone())).unwrap();
                
                self.turn_phase = Don;
            },
            Don => {
                debug!("(TURN) [DON]");
                current_player.draw_don(2);
                comms_tx.send(ServerMessage::PlayerDataPayload(current_player.clone())).unwrap();
                self.turn_phase = Main;
            },
            Main => {
                debug!("(TURN) [MAIN]");
                let player_action = comms_rx.try_recv();
                match player_action {
                    Ok(PlayerAction::NoAction) => { 
                        self.turn_phase = End; 
                        return;
                    }
                    Ok(_) => { },
                    Err(e) => { },
                }
                comms_tx.send(ServerMessage::PlayerDataPayload(current_player.clone())).unwrap();
                comms_tx.send(ServerMessage::TakeMainAction).unwrap();
            },
            BattleAttackStep => {},
            BattleBlockStep => {},
            BattleCounterStep => {},
            BattleDamageStep => {},
            BattleEnd => {},
            End => {
                self.turn_n += 1;
                self.turn = match self.turn {
                    Turn::P1 => Turn::P2,
                    Turn::P2 => Turn::P1,
                };
                self.turn_phase = Refresh;
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TurnPhase {
    Refresh,
    Draw,
    Don,
    Main,
    BattleAttackStep,
    BattleBlockStep,
    BattleCounterStep,
    BattleDamageStep,
    BattleEnd,
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
            let val: String;
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
            match self.0 {
                0 => (),
                i => write!(f, "{i}")?,
            }
            Ok(())
        }
    }

    impl fmt::Display for CardCategory {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            use CardCategory::*;
            let val: String;
            match self {
                Leader(_) => val = "Leader".into(),
                Character => val = "Character".into(),
                Event => val = "Event".into(),
                Stage => val = "Stage".into(),
                Don => val = "DON!! CARD".into(),
            }
            write!(f, "{}", val)?;
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
            let val: String;
            match self {
                OnPlay => val = "[On Play]".into(),
                WhenAttacking => val = "[When Attacking]".into(),
                ActivateMain => val = "[Activate:Main]".into(),
                Main => val = "[Main]".into(),
                CounterPhase => val = "[Counter]".into(),
                DuringTurn => val = "".into(),
                Trigger => val = "[Trigger]".into(),
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
                Blocker => write!(f, "<Blocker>")?,
                TimedEffect(timing, cost, effect) => {
                    write!(f, "{} {} ", timing, cost)?;
                    for effect in effect.iter() {
                        write!(f, "{} ", effect)?;
                    }
                },
                Draw(i) => write!(f, "Draw {}", i)?,
                GiveOtherCardPower(i) => write!(f, "Give your Leader or 1 of your Characters other than this card +{i} power during this turn.")?,
                GiveRestedDon(i) => write!(f, "Give this Leader or 1 of your Characters {i} rested DON!! card(s).")?,
                KnockOutWithPowerLessThan(i) => write!(f, "K.O. 1 of your opponent's Characters with a power of {i} or less.")?,
                OncePerTurn => write!(f, "Once Per Turn")?,
                OpponentNoBlocker(condition) => {
                    match condition {
                        Condition::None => write!(f, "Your opponent cannot activate <Blocker> during this battle.")?,
                        Condition::PowerAndAbove(i) => write!(f, "Your opponent cannot activate <Blocker> of {i} or higher Power Characters during this battle.")?,
                    }
                },
                Rush => write!(f, "<Rush>")?,
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
            write!(
                f,
                "| {}                          {} ",
                self.cost,
                self.power.unwrap_or(CardPower(0))
            )?;
            for att in self.attribute.iter() {
                write!(f, "{}", att)?;
            }
            write!(f, " \n")?;
            write!(f, "|                                    \n")?;
            write!(
                f,
                "|  {}                              \n",
                self.counter_power.unwrap_or(CounterPower(0))
            )?;
            write!(f, "|                                    \n")?;
            for effect in self.effects.iter() {
                write!(f, "|  {}  \n", effect)?;
            }
            for _ in 0..(5 - self.effects.len()) {
                write!(f, "|                                    \n")?;
            }
            write!(f, "|               {}               \n", self.category)?;
            write!(f, "|               {}               \n", self.name)?;
            write!(f, "| ")?;
            for c in self.color.iter() {
                write!(f, "{} ", c)?;
            }
            write!(f, "  ")?;
            for t in self.types.iter() {
                write!(f, " {} ", t)?;
            }
            write!(f, "        \n")?;
            write!(f, "---------------------------------------\n")?;

            Ok(())
        }
    }
}
