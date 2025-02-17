use super::{card::*, player::*, *};

pub struct PlayerArea<'a> {
    pub player: &'a mut Player,
    pub life: &'a mut Deck,
    pub stage: &'a mut Deck,
    pub character: &'a mut Deck,
    pub rested_character: &'a mut Deck,
    pub active_don: &'a mut Deck,
    pub rested_don: &'a mut Deck,
}

impl PlayerArea<'_> {
    pub fn process_knock_out(&mut self, i: usize) {
        let card = self.character.remove(i);
        self.player.trash.push(card);
    }

    pub fn count_don_in_play(&self) -> i32 {
        self.active_don.len() as i32 + self.rested_don.len() as i32 + self.character.iter().map(|c| c.attached_don.len() as i32).sum::<i32>()
    }
}