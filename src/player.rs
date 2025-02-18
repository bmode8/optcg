use log::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use super::card::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub leader: Card,
    pub main_deck: Deck,
    pub don_deck: Deck,
    pub hand: Deck,
    pub trash: Deck,
}

/// Game related implementations.
impl Player {
    pub fn empty() -> Player {
        Player {
            name: String::new(),
            leader: Card::new(
                String::new(),
                String::new(),
                String::new(),
                CardCost(0),
                CardCategory::Character,
                None,
                None,
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Facing::FaceDown,
            ),
            main_deck: Deck::new(),
            don_deck: Deck::new(),
            hand: Deck::new(),
            trash: Deck::new(),
        }
    }

    /// Draws `n` cards from the main deck and places them in the player's hand.
    /// Importantly, this is implemented by `pop`ping the main deck, which means
    /// that the top of the deck is the end of the `Deck`.
    pub fn draw(mut self, n: i32) -> Result<Self, Self> {
        for _ in 0..n {
            let drawn_card = self.main_deck.pop();

            if drawn_card.is_none() {
                error!("COULD NOT DRAW CARD");
                return Err(self);
            }
            let mut drawn_card = drawn_card.unwrap();
            let mut drawn_card = drawn_card.set_faceup();
            debug!("DRAWN CARD: {}", drawn_card);
            self.hand.push(drawn_card);
        }

        Ok(self)
    }

    pub fn draw_don(mut self, n: i32) -> (Self, Deck) {
        let mut drawn_don = Deck::new();
        for _ in 0..n {
            let drawn_card = self.don_deck.pop();

            if drawn_card.is_none() {
                return (self, drawn_don);
            }

            drawn_don.push(drawn_card.unwrap().set_faceup());
        }

        (self, drawn_don)
    }

    pub fn draw_out(mut self, n: i32) -> Result<(Self, Deck), ()> {
        let mut drawn_out = Deck::new();
        for _ in 0..n {
            let drawn_card = self.main_deck.pop();

            if drawn_card.is_none() {
                return Err(());
            }

            drawn_out.push(drawn_card.unwrap());
        }

        Ok((self, drawn_out))
    }

    pub fn topdeck_hand(mut self) -> Self {
        self.main_deck.append(&mut self.hand);

        self
    }

    pub fn shuffle(mut self, rng: &mut StdRng) -> Self{
        self.main_deck.shuffle(rng);

        self
    }

    pub fn turn_hand_faceup(mut self) -> Self {
        let current_hand = self.hand;

        self.hand = current_hand.into_iter().map(|c| c.set_faceup()).collect();

        self
    }
}

/// Non-game related implementations.
impl Player {
    pub fn public_clone(&self) -> Box<Player> {
        Box::new(Player {
            name: self.name.clone(),
            leader: self.leader.clone(),
            main_deck: self.main_deck.clone(),
            don_deck: self.don_deck.clone(),
            hand: self.hand.clone().set_facedown(),
            trash: self.trash.clone(),
        })
    }
}
