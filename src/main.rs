#![allow(unused)]

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
    
    println!("{:?}", franky);
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

#[derive(Debug, Clone, Copy)]
pub struct CardCost(i32);

#[derive(Debug, Clone, Copy)]
pub struct CardPower(i32);

#[derive(Debug, Clone, Copy)]
pub struct CounterPower(i32);
 
#[derive(Debug, Clone, Copy)]
pub enum CardCategory {
    Leader,
    Character,
    Event,
    Stage,
    Don,
}

#[derive(Debug, Clone, Copy)]
pub enum Attribute {
    Slash,
    Strike,
    Ranged,
    Special,
    Wisdom,
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
}

///effects: vec![Effect::Blocker, Effect::TimedEffect(MainPhaseAction::PlayCard, EffectCost::MinusDon(2), Effect::Draw(2))]

#[derive(Debug, Clone)]
pub enum Effect {
    Blocker,
    TimedEffect(Timing, EffectCost, Box<Effect>),
    Draw(i32),
}

#[derive(Debug, Clone)]
pub enum EffectCost {
    MinusDon(i32),
    RestDon(i32),
    DonAttached(Box<Card>, i32)
}