use std::fmt;

use serde::{Deserialize, Serialize};

pub mod card;
pub mod game;
pub mod player;

use card::*;
use game::*;
use player::*;

pub fn print_hand(hand: &Deck) {
    for (i, card) in hand.iter().enumerate() {
        println!("{i}");

        print!("{}", card.cost);
        for color in card.color.iter() {
            print!("{}", color);
        }

        println!(" {}", card.name);
        println!("{}", card.category);
        if card.types.len() > 0 {
            print!("{}", card.types[0]);
            for t in card.types.iter().skip(1) {
                print!("/{}", t);
            }
        }

        println!();
        match card.power {
            Some(power) => print!("+{} ", power),
            None => (),
        }

        if card.attribute.len() > 0 {
            print!("{}", card.attribute[0]);
            for att in card.attribute.iter().skip(1) {
                print!("/{}", att);
            }
        }
        println!();
        for effect in card.effects.iter() {
            println!("{} ", effect);
        }

        match card.counter_power {
            Some(power) => println!("Counter +{} ", power),
            None => (),
        };

        println!();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlayerAction {
    Idle, // Heartbeat for the client to send the server.
    ReportDeck(String),
    TakeMulligan,
    NoAction,
    MainActivateCardEffect(usize),
    MainPlayCard(usize),
    MainAttachDon(usize),
    MainBattle(usize),
    End,
    TargetOpposingCharacter(usize),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    Connected,
    PlayerId(Turn),
    RequestDeck,
    QueryMulligan,
    TakeMainAction,
    InsufficientDon,
    CannotPlayCounterEventDuringMainPhase,
    QueryTargetOpposingCharacter,
    NoTargetsMeetConditions,
    PlayerDataPayload(Box<Player>),
    OtherPlayerDataPayload(Box<Player>),
    PublicPlayfieldStateDataPayload(Box<PublicPlayfieldState>),
}

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
            Counter => val = "[Counter]".into(),
            DuringTurn => val = "".into(),
            Trigger => val = "[Trigger]".into(),
            Always => val = "".into(),
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
            KnockOutWithPowerEqualOrLessThan(i) => write!(f, "K.O. 1 of your opponent's Characters with a power of {i} or less.")?,
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
