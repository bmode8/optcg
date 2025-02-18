use super::{card::*, player::*, *};

pub struct PlayerArea {
    pub player: Player,
    pub life: Deck,
    pub stage: Deck,
    pub character: Deck,
    pub rested_character: Deck,
    pub active_don: Deck,
    pub rested_don: Deck,
}

impl PlayerArea {
    pub fn process_knock_out(mut self, i: usize) -> Self{
        let card = self.character.remove(i);
        self.player.trash.push(card);
        self
    }

    pub fn count_don_in_play(&self) -> i32 {
        self.active_don.len() as i32 + self.rested_don.len() as i32 + self.character.iter().map(|c| c.attached_don.len() as i32).sum::<i32>()
    }
}