#![allow(unused)]

use futures::prelude::*;
use log::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio_serde::formats::*;
use tokio_serde::Framed;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use super::{card::*, player::*, player_area::*, *};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Turn {
    P1,
    P2,
}

pub type PlayerId = Turn;

pub struct GameServer<'stream> {
    pub game: GameState,
    pub p1_client: PlayerClient<'stream>,
    pub p2_client: PlayerClient<'stream>,
}

pub struct PlayerClient<'stream> {
    pub player: Box<Player>,
    pub reader: Framed<
        FramedRead<ReadHalf<'stream>, LengthDelimitedCodec>,
        Value,
        Value,
        Json<Value, Value>,
    >,
    pub writer: Framed<
        FramedWrite<WriteHalf<'stream>, LengthDelimitedCodec>,
        Value,
        Value,
        Json<Value, Value>,
    >,
}

impl<'stream> PlayerClient<'stream> {
    pub async fn send_message(&mut self, action: ServerMessage) {
        self.writer
            .send(serde_json::from_str(serde_json::to_string(&action).unwrap().as_str()).unwrap())
            .await
            .unwrap();
    }

    pub async fn receive_next_nonidle_action(&mut self) -> PlayerAction {
        loop {
            let action: PlayerAction =
                serde_json::from_value(self.reader.try_next().await.unwrap().unwrap()).unwrap();

            match action {
                PlayerAction::Idle => {}
                _ => return action,
            }
        }
    }
}

#[derive(Debug)]
pub struct GameState {
    pub turn: Turn,
    pub turn_phase: TurnPhase,
    pub turn_n: i32,
    pub player_1: Player,
    pub player_2: Player,
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
    pub rng: StdRng,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicPlayfieldState {
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
}

impl PublicPlayfieldState {
    pub fn empty() -> PublicPlayfieldState {
        PublicPlayfieldState {
            p1_life_area: Deck::new(),
            p2_life_area: Deck::new(),
            p1_stage_area: Deck::new(),
            p2_stage_area: Deck::new(),
            p1_character_area: Deck::new(),
            p2_character_area: Deck::new(),
            p1_rested_character_area: Deck::new(),
            p2_rested_character_area: Deck::new(),
            p1_active_don_area: Deck::new(),
            p2_active_don_area: Deck::new(),
            p1_rested_don_area: Deck::new(),
            p2_rested_don_area: Deck::new(),
        }
    }

    pub fn new(
        p1_life_area: Deck,
        p2_life_area: Deck,
        p1_stage_area: Deck,
        p2_stage_area: Deck,
        p1_character_area: Deck,
        p2_character_area: Deck,
        p1_rested_character_area: Deck,
        p2_rested_character_area: Deck,
        p1_active_don_area: Deck,
        p2_active_don_area: Deck,
        p1_rested_don_area: Deck,
        p2_rested_don_area: Deck,
    ) -> PublicPlayfieldState {
        PublicPlayfieldState {
            p1_life_area,
            p2_life_area,
            p1_stage_area,
            p2_stage_area,
            p1_character_area,
            p2_character_area,
            p1_rested_character_area,
            p2_rested_character_area,
            p1_active_don_area,
            p2_active_don_area,
            p1_rested_don_area,
            p2_rested_don_area,
        }
    }

    pub fn from_playfield(playfield: &GameState) -> PublicPlayfieldState {
        PublicPlayfieldState {
            p1_life_area: playfield.p1_life_area.clone(),
            p2_life_area: playfield.p2_life_area.clone(),
            p1_stage_area: playfield.p1_stage_area.clone(),
            p2_stage_area: playfield.p2_stage_area.clone(),
            p1_character_area: playfield.p1_character_area.clone(),
            p2_character_area: playfield.p2_character_area.clone(),
            p1_rested_character_area: playfield.p1_rested_character_area.clone(),
            p2_rested_character_area: playfield.p2_rested_character_area.clone(),
            p1_active_don_area: playfield.p1_active_don_area.clone(),
            p2_active_don_area: playfield.p2_active_don_area.clone(),
            p1_rested_don_area: playfield.p1_rested_don_area.clone(),
            p2_rested_don_area: playfield.p2_rested_don_area.clone(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct TurnInfo {
    pub turn: Turn,
    pub turn_phase: TurnPhase,
    pub turn_n: i32,
}

impl GameState {
    async fn sync_data<'stream>(
        current_player_client: &mut PlayerClient<'stream>,
        other_player_client: &mut PlayerClient<'stream>,
        current_player: &Box<Player>,
        other_player: &Box<Player>,
        public_field_state: PublicPlayfieldState,
    ) {
        current_player_client
            .send_message(ServerMessage::PlayerDataPayload(current_player.clone()))
            .await;
        current_player_client
            .send_message(ServerMessage::OtherPlayerDataPayload(
                other_player.public_clone(),
            ))
            .await;
        current_player_client
            .send_message(ServerMessage::PublicPlayfieldStateDataPayload(Box::new(
                public_field_state.clone(),
            )))
            .await;
        other_player_client
            .send_message(ServerMessage::PlayerDataPayload(other_player.clone()))
            .await;
        other_player_client
            .send_message(ServerMessage::OtherPlayerDataPayload(
                current_player.public_clone(),
            ))
            .await;
        other_player_client
            .send_message(ServerMessage::PublicPlayfieldStateDataPayload(Box::new(
                public_field_state,
            )))
            .await;
    }

    pub async fn setup<'stream>(
        mut player_1: Player,
        mut player_2: Player,
        p1_client: &mut PlayerClient<'stream>,
        p2_client: &mut PlayerClient<'stream>,
    ) -> GameState {
        let mut rng = StdRng::seed_from_u64(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u64,
        );

        let player_1 = player_1.shuffle(&mut rng).draw(5).unwrap();
        let player_2 = player_2.shuffle(&mut rng).draw(5).unwrap();

        let public_playfield_state = Box::new(PublicPlayfieldState::empty());

        p1_client
            .send_message(ServerMessage::PlayerDataPayload(Box::new(player_1.clone())))
            .await;
        p2_client
            .send_message(ServerMessage::PlayerDataPayload(Box::new(player_2.clone())))
            .await;
        p1_client
            .send_message(ServerMessage::OtherPlayerDataPayload(
                player_2.public_clone(),
            ))
            .await;
        p2_client
            .send_message(ServerMessage::OtherPlayerDataPayload(
                player_1.public_clone(),
            ))
            .await;
        p1_client
            .send_message(ServerMessage::PublicPlayfieldStateDataPayload(
                public_playfield_state.clone(),
            ))
            .await;
        p2_client
            .send_message(ServerMessage::PublicPlayfieldStateDataPayload(
                public_playfield_state.clone(),
            ))
            .await;

        // Query Mulligan
        p1_client.send_message(ServerMessage::QueryMulligan).await;
        let p1_mulligan = p1_client.receive_next_nonidle_action().await;

        let player_1 = if let PlayerAction::TakeMulligan = p1_mulligan {
            let p = player_1.topdeck_hand().shuffle(&mut rng).draw(5).unwrap();
            p1_client
                .send_message(ServerMessage::PlayerDataPayload(Box::new(p.clone())))
                .await;

            p
        } else { player_1 };

        p2_client.send_message(ServerMessage::QueryMulligan).await;
        let p2_mulligan = p2_client.receive_next_nonidle_action().await;

        let player_2 = if let PlayerAction::TakeMulligan = p2_mulligan {
            let p = player_2.topdeck_hand().shuffle(&mut rng).draw(5).unwrap();
            p2_client
                .send_message(ServerMessage::PlayerDataPayload(Box::new(p.clone())))
                .await;
            p
        } else { player_2 };

        let p1_life_val = player_1.leader.life();
        let p2_life_val = player_2.leader.life();

        let (player_1, p1_life) = player_1.draw_out(p1_life_val).unwrap();
        let (player_2, p2_life) = player_2.draw_out(p2_life_val).unwrap();

        // Begin turn 1.
        let (player_1, p1_don) = player_1.draw_don(1);

        p1_client
            .send_message(ServerMessage::PlayerDataPayload(Box::new(player_1.clone())))
            .await;
        p2_client
            .send_message(ServerMessage::PlayerDataPayload(Box::new(player_2.clone())))
            .await;
        p1_client
            .send_message(ServerMessage::OtherPlayerDataPayload(
                player_2.public_clone(),
            ))
            .await;
        p2_client
            .send_message(ServerMessage::OtherPlayerDataPayload(
                player_1.public_clone(),
            ))
            .await;

        GameState {
            turn: Turn::P1,
            turn_phase: TurnPhase::Main,
            turn_n: 1,
            player_1,
            player_2,
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
        }
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
            return Some(PlayerId::P1);
        } else if p2_deck_len == 0 {
            return Some(PlayerId::P2);
        }

        None
    }

    pub fn trigger_loser() -> Option<PlayerId> {
        todo!()
    }

    pub fn split_into_player_areas(self) -> (PlayerArea, PlayerArea, TurnInfo, StdRng) {
        match self.turn {
            Turn::P1 => (
                PlayerArea {
                    player: self.player_1,
                    life: self.p1_life_area,
                    stage: self.p1_stage_area,
                    character: self.p1_character_area,
                    rested_character: self.p1_rested_character_area,
                    active_don: self.p1_active_don_area,
                    rested_don: self.p1_rested_don_area,
                },
                PlayerArea {
                    player: self.player_2,
                    life: self.p2_life_area,
                    stage: self.p2_stage_area,
                    character: self.p2_character_area,
                    rested_character: self.p2_rested_character_area,
                    active_don: self.p2_active_don_area,
                    rested_don: self.p2_rested_don_area,
                },
                TurnInfo {
                    turn: self.turn,
                    turn_phase: self.turn_phase,
                    turn_n: self.turn_n,
                },
                self.rng
            ),
            Turn::P2 => (
                PlayerArea {
                    player: self.player_2,
                    life: self.p2_life_area,
                    stage: self.p2_stage_area,
                    character: self.p2_character_area,
                    rested_character: self.p2_rested_character_area,
                    active_don: self.p2_active_don_area,
                    rested_don: self.p2_rested_don_area,
                },
                PlayerArea {
                    player: self.player_1,
                    life: self.p1_life_area,
                    stage: self.p1_stage_area,
                    character: self.p1_character_area,
                    rested_character: self.p1_rested_character_area,
                    active_don: self.p1_active_don_area,
                    rested_don: self.p1_rested_don_area,
                },
                TurnInfo {
                    turn: self.turn,
                    turn_phase: self.turn_phase,
                    turn_n: self.turn_n,
                },
                self.rng
            ),
        }
    }

    fn recombine_player_areas_into_gamestate(current_player_area: PlayerArea, other_player_area: PlayerArea, turn_info: TurnInfo, rng: StdRng) -> Self {
        let (player_1_area, player_2_area) = match turn_info.turn {
            Turn::P1 => (current_player_area, other_player_area),
            Turn::P2 => (other_player_area, current_player_area),
        };

        Self {
            turn: turn_info.turn,
            turn_phase: turn_info.turn_phase,
            turn_n: turn_info.turn_n,
            player_1: player_1_area.player,
            player_2: player_2_area.player,
            p1_life_area: player_1_area.life,
            p2_life_area: player_2_area.life,
            p1_stage_area: player_1_area.stage,
            p2_stage_area: player_2_area.stage,
            p1_character_area: player_1_area.character,
            p2_character_area: player_2_area.character,
            p1_rested_character_area: player_1_area.rested_character,
            p2_rested_character_area: player_2_area.rested_character,
            p1_active_don_area: player_1_area.active_don,
            p2_active_don_area: player_2_area.active_don,
            p1_rested_don_area: player_1_area.rested_don,
            p2_rested_don_area: player_2_area.rested_don,
            rng,
        }
    }

    pub fn public_playfield_state(
        turn_info: TurnInfo,
        current_player_area: &PlayerArea,
        other_player_area: &PlayerArea,
    ) -> PublicPlayfieldState {
        let public_state;
        match turn_info.turn {
            Turn::P1 => {
                public_state = PublicPlayfieldState::new(
                    current_player_area.life.clone(),
                    other_player_area.life.clone(),
                    current_player_area.stage.clone(),
                    other_player_area.stage.clone(),
                    current_player_area.character.clone(),
                    other_player_area.character.clone(),
                    current_player_area.rested_character.clone(),
                    other_player_area.rested_character.clone(),
                    current_player_area.active_don.clone(),
                    other_player_area.active_don.clone(),
                    current_player_area.rested_don.clone(),
                    other_player_area.rested_don.clone(),
                );
            }
            Turn::P2 => {
                public_state = PublicPlayfieldState::new(
                    other_player_area.life.clone(),
                    current_player_area.life.clone(),
                    other_player_area.stage.clone(),
                    current_player_area.stage.clone(),
                    other_player_area.character.clone(),
                    current_player_area.character.clone(),
                    other_player_area.rested_character.clone(),
                    current_player_area.rested_character.clone(),
                    other_player_area.active_don.clone(),
                    current_player_area.active_don.clone(),
                    other_player_area.rested_don.clone(),
                    current_player_area.rested_don.clone(),
                );
            }
        };

        public_state
    }

    pub async fn step<'stream>(
        mut self,
        p1_client: &mut PlayerClient<'stream>,
        p2_client: &mut PlayerClient<'stream>,
    ) -> Self {
        use TurnPhase::*;

        debug!("{:?}, {:?}, {:?}", self.turn, self.turn_phase, self.turn_n);

        let (mut current_player_area, mut other_player_area, mut turn_info, mut rng) = 
            self.split_into_player_areas();

        let (mut current_player_client, mut other_player_client) = match turn_info.turn {
            Turn::P1 => (p1_client, p2_client),
            Turn::P2 => (p2_client, p1_client),
        };

        // Behold! A state machine!
        match turn_info.turn_phase {
            Refresh => {
                let (current_player_area, other_player_area) = Self::refresh_step(
                    current_player_area,
                    other_player_area,
                    &mut current_player_client,
                    &mut other_player_client,
                    turn_info,
                )
                .await;

                turn_info.turn_phase = Draw;
                self = Self::recombine_player_areas_into_gamestate(current_player_area, other_player_area, turn_info, rng);
            }
            Draw => {
                let (current_player_area, other_player_area) = Self::draw_step(
                    current_player_area,
                    other_player_area,
                    &mut current_player_client,
                    &mut other_player_client,
                    turn_info,
                )
                .await;
                turn_info.turn_phase = Don;
                self = Self::recombine_player_areas_into_gamestate(current_player_area, other_player_area, turn_info, rng);
            }
            Don => {
                let (current_player_area, other_player_area) = Self::don_step(
                    current_player_area,
                    other_player_area,
                    &mut current_player_client,
                    &mut other_player_client,
                    turn_info,
                )
                .await;
                turn_info.turn_phase = Main;
                self = Self::recombine_player_areas_into_gamestate(current_player_area, other_player_area, turn_info, rng);
            }
            Main => {
                let (current_player_area, other_player_area, next_turn_phase) = Self::main_step(
                    current_player_area,
                    other_player_area,
                    &mut current_player_client,
                    &mut other_player_client,
                    turn_info,
                )
                .await;
                self = Self::recombine_player_areas_into_gamestate(current_player_area, other_player_area, turn_info, rng);
            }
            BattleAttackStep => {
                self = Self::recombine_player_areas_into_gamestate(current_player_area, other_player_area, turn_info, rng);
            }
            BattleBlockStep => {
                self = Self::recombine_player_areas_into_gamestate(current_player_area, other_player_area, turn_info, rng);
            }
            BattleCounterStep => {
                self = Self::recombine_player_areas_into_gamestate(current_player_area, other_player_area, turn_info, rng);
            }
            BattleDamageStep => {
                self = Self::recombine_player_areas_into_gamestate(current_player_area, other_player_area, turn_info, rng);
            }
            BattleEnd => {
                self = Self::recombine_player_areas_into_gamestate(current_player_area, other_player_area, turn_info, rng);
            }
            End => {
                self = Self::recombine_player_areas_into_gamestate(current_player_area, other_player_area, turn_info, rng);
                debug!("(TURN) [END]");
                self.turn_n += 1;
                self.turn = match turn_info.turn {
                    Turn::P1 => Turn::P2,
                    Turn::P2 => Turn::P1,
                };
                self.turn_phase = Refresh;
            }
        }

        self
    }

    pub async fn refresh_step<'stream>(
        mut current_player_area: PlayerArea,
        mut other_player_area: PlayerArea,
        current_player_client: &mut PlayerClient<'stream>,
        other_player_client: &mut PlayerClient<'stream>,
        turn_info: TurnInfo,
    ) -> (PlayerArea, PlayerArea) {
        debug!("(TURN) [REFRESH]");
        current_player_area
            .active_don
            .append(&mut current_player_area.rested_don);
        current_player_area
            .character
            .append(&mut current_player_area.rested_character);
        for card in current_player_area.character.iter_mut() {
            current_player_area
                .active_don
                .append(&mut card.attached_don);
        }

        let public_state =
            Self::public_playfield_state(turn_info, &current_player_area, &other_player_area);

        Self::sync_data(
            current_player_client,
            other_player_client,
            &Box::new(current_player_area.player.clone()),
            &Box::new(other_player_area.player.clone()),
            public_state,
        )
        .await;

        (current_player_area, other_player_area)
    }

    pub async fn draw_step<'stream>(
        mut current_player_area: PlayerArea,
        mut other_player_area: PlayerArea,
        current_player_client: &mut PlayerClient<'stream>,
        other_player_client: &mut PlayerClient<'stream>,
        turn_info: TurnInfo,
    ) -> (PlayerArea, PlayerArea) {
        debug!("(TURN) [DRAW]");
        let res = current_player_area.player.draw(1);
        match res {
            Ok(p) => {current_player_area.player = p;},
            Err(p) => {
                current_player_area.player = p;
                return (current_player_area, other_player_area);
            }
        }

        let public_state =
            Self::public_playfield_state(turn_info, &current_player_area, &other_player_area);

        Self::sync_data(
            current_player_client,
            other_player_client,
            &Box::new(current_player_area.player.clone()),
            &Box::new(other_player_area.player.clone()),
            public_state,
        )
        .await;

        (current_player_area, other_player_area)
    }

    pub async fn don_step<'stream>(
        mut current_player_area: PlayerArea,
        mut other_player_area: PlayerArea,
        current_player_client: &mut PlayerClient<'stream>,
        other_player_client: &mut PlayerClient<'stream>,
        turn_info: TurnInfo,
    ) -> (PlayerArea, PlayerArea) {
        debug!("(TURN) [DON]");
        let (p, mut drawn_don) = current_player_area.player.draw_don(2);
        current_player_area.player = p;
        current_player_area.active_don.append(&mut drawn_don);

        let public_state = Self::public_playfield_state(
            turn_info,
            &current_player_area,
            &other_player_area,
        );

        Self::sync_data(
            current_player_client,
            other_player_client,
            &Box::new(current_player_area.player.clone()),
            &Box::new(other_player_area.player.clone()),
            public_state,
        )
        .await;

        (current_player_area, other_player_area)
    }

    pub async fn main_step<'stream>(
        mut current_player_area: PlayerArea,
        mut other_player_area: PlayerArea,
        current_player_client: &mut PlayerClient<'stream>,
        other_player_client: &mut PlayerClient<'stream>,
        turn_info: TurnInfo,
    ) -> (PlayerArea, PlayerArea, TurnPhase) {
        debug!("(TURN) [MAIN]");

        let public_state = Self::public_playfield_state(
            turn_info,
            &current_player_area,
            &other_player_area,
        );

        Self::sync_data(
            current_player_client,
            other_player_client,
            &Box::new(current_player_area.player.clone()),
            &Box::new(other_player_area.player.clone()),
            public_state,
        )
        .await;

        debug!(
            "Sending {:?} to {}",
            ServerMessage::TakeMainAction,
            current_player_area.player.name
        );
        current_player_client
            .send_message(ServerMessage::TakeMainAction)
            .await;
        let player_action = current_player_client.receive_next_nonidle_action().await;
        debug!("Received {:?}", player_action);
        match player_action {
            PlayerAction::End => {
                return (current_player_area, other_player_area, TurnPhase::End);
            }
            PlayerAction::MainPlayCard(c) => {
                let card = current_player_area.player.hand.remove(c);

                // Can you pay for it?
                let cost = card.cost.0;
                if cost as usize > current_player_area.active_don.len() {
                    current_player_client
                        .send_message(ServerMessage::InsufficientDon)
                        .await;
                    current_player_area.player.hand.insert(c, card);
                    return (current_player_area, other_player_area, TurnPhase::Main);
                }

                // Is it an Event card with Counter Timing?
                match card.category {
                    CardCategory::Event => {
                        match card.effects.iter().take(1).next().unwrap() {
                            Effect::TimedEffect(timing, _, _) => {
                                match timing {
                                    Timing::Main => {} // fine
                                    Timing::Counter => {
                                        current_player_client.send_message(ServerMessage::CannotPlayCounterEventDuringMainPhase).await;
                                        return (current_player_area, other_player_area, TurnPhase::Main);
                                    }
                                    _ => {} // also fine
                                }
                            }
                            _ => unreachable!(), // not something that should happen.
                        }
                    }
                    _ => {} // not an event card.
                }

                // Pay for it if it isn't free.
                if cost > 0 {
                    let mut don_to_pay = vec![];
                    for _ in 0..cost {
                        don_to_pay.push(current_player_area.active_don.pop().unwrap());
                    }
                    current_player_area.rested_don.append(&mut don_to_pay);
                }

                match card.category {
                    // already made sure this can be played now.
                    CardCategory::Event => {
                        let main_effect = card.effects.iter().next().unwrap();
                        match main_effect {
                            Effect::TimedEffect(timing, effect_cost, effect) => {
                                match timing {
                                    Timing::Main => {
                                        for effect in effect.iter() {
                                            match *effect {
                                                Effect::Blocker => {

                                                }
                                                Effect::Draw(n) => {
                                                    let res = current_player_area.player.draw(n);
                                                    match res {
                                                        Ok(p) => {current_player_area.player = p;},
                                                        Err(p) => {
                                                            current_player_area.player = p;
                                                            Self::trigger_loser();
                                                        }
                                                    }
                                                }
                                                Effect::GiveOtherCardPower(x) => {

                                                }
                                                Effect::GiveRestedDon(n) => {

                                                }
                                                Effect::KnockOutWithPowerEqualOrLessThan(x) => {
                                                    // are there any valid targets on the field?
                                                    if other_player_area.character.iter().filter(|c| c.power.unwrap().0 <= x).count() == 0 {
                                                        current_player_client.send_message(ServerMessage::NoTargetsMeetConditions).await;
                                                        continue;
                                                    }

                                                    loop {
                                                        // ask player to select an opponent's character with power less than x to knock out.
                                                        current_player_client.send_message(ServerMessage::QueryTargetOpposingCharacter).await;
                                                        let attempted_target = current_player_client.receive_next_nonidle_action().await;
                                                        match attempted_target {
                                                            PlayerAction::TargetOpposingCharacter(i) => {
                                                                // if they select one that is too powerful, loop back.
                                                                if other_player_area.character[i].power.unwrap().0 < x {
                                                                    continue;
                                                                }

                                                                other_player_area = other_player_area.process_knock_out(i);

                                                                let public_state = Self::public_playfield_state(turn_info, &current_player_area, &other_player_area);

                                                                Self::sync_data(
                                                                    current_player_client,
                                                                    other_player_client,
                                                                    &Box::new(current_player_area.player.clone()),
                                                                    &Box::new(other_player_area.player.clone()),
                                                                    public_state,
                                                                )
                                                                .await;
                                                            },
                                                            _ => { }
                                                        }
                                                    }
                                                    // process the knock out.
                                                }
                                                Effect::OncePerTurn => {

                                                }
                                                Effect::OpponentNoBlocker(condition) => {

                                                }
                                                Effect::PlayCard => {

                                                }
                                                Effect::PlusPower(x) => {

                                                }
                                                Effect::PlusPowerForBattle(x) => {

                                                }
                                                Effect::Rush => {

                                                }
                                                _ => unreachable!(), // shouldn't have another TimedEffect inside the TimedEffect.
                                            }
                                        }
                                    }
                                    _ => unreachable!(), // we've accounted for all other possibilities here.
                                }
                            }
                            _ => unreachable!(), // effects on event cards should be contained in a TimedEffect with Main timing.
                        }

                        // played and processed event card goes in the trash after.
                        current_player_area.player.trash.push(card);
                    }
                    CardCategory::Stage => {}
                    CardCategory::Character => {
                        // process any `OnPlay` effects.
                        for effect in card.effects.iter() {
                            match effect {
                                Effect::TimedEffect(timing, effect_cost, effect) => {
                                    match timing {
                                        Timing::OnPlay => {
                                            // can you pay for it? board updated to reflect payment in this match.
                                            match effect_cost {
                                                EffectCost::MinusDon(n) => {
                                                    // are there enough don? if not, tell player and move on from activating the effect and add the card to character area.
                                                    let num_don_in_play = current_player_area.count_don_in_play();
                                                    if num_don_in_play < *n {
                                                        current_player_client.send_message(ServerMessage::InsufficientDon).await;
                                                        continue;
                                                    }
                                                    // if there are enough total don available, let player select `n` to put back in don deck or skip effect.
                                                        // how to do this? query player for `n` targets, make client package them up and send them here.
                                                        // maybe just a special enum for the vec of locations to take the don!! from, like Leader, C1, etc., C5, Rested, Active
                                                    // then put those cards back in the don deck.
                                                    // continue to processing the effect.
                                                }
                                                EffectCost::DonAttached(n) => {
                                                    unreachable!()
                                                }
                                                EffectCost::RestDon(n) => {
                                                    // are there enough active don? if not, tell player and move on from activating the effect and add the card to character area.
                                                    // prompt player for effect choice.
                                                    // if they choose to activate the effect, rest `n` don cards.
                                                    // continue to processing the effect.
                                                }
                                                EffectCost::Zero => {} // the intended representation of no additional cost.
                                            }

                                            for e in effect.iter() {
                                                match e {
                                                    Effect::Blocker => {}
                                                    Effect::Draw(n) => {}
                                                    Effect::GiveOtherCardPower(x) => {}
                                                    Effect::GiveRestedDon(n) => {
                                                        let rested_don_number = current_player_area.rested_don.len();
                                                        let mut don_to_give = vec![];
                                                        match rested_don_number {
                                                            0 => { continue }
                                                            1 => { don_to_give.push(current_player_area.rested_don.pop().unwrap()); }
                                                            _ => {
                                                                don_to_give.push(current_player_area.rested_don.pop().unwrap());
                                                                don_to_give.push(current_player_area.rested_don.pop().unwrap());
                                                            }
                                                        }
                                                        // prompt player for target in their own area.
                                                        loop {
                                                            current_player_client.send_message(ServerMessage::QueryTargetSelfCharacterOrLeader).await;
                                                            let attempted_target = current_player_client.receive_next_nonidle_action().await;
                                                            match attempted_target {
                                                                PlayerAction::TargetSelfCharacterOrLeader(c) => {
                                                                    match c {
                                                                        'l' => { current_player_area.player.leader.attached_don.append(&mut don_to_give); break; }
                                                                        _ => {
                                                                            let char_idx = c.to_digit(10).unwrap() as usize;
                                                                            if char_idx > current_player_area.character.len() - 1 {
                                                                                current_player_client.send_message(ServerMessage::InvalidTarget).await;
                                                                                continue;
                                                                            }
                                                                            current_player_area.character[char_idx].attached_don.append(&mut don_to_give);
                                                                            break;
                                                                        }
                                                                    }
                                                                }
                                                                _ => { }
                                                            }
                                                        }
                                                        // give those rested don cards to the target.
                                                    }
                                                    Effect::KnockOutWithPowerEqualOrLessThan(x) => {}
                                                    Effect::OncePerTurn => {}
                                                    Effect::OpponentNoBlocker(condition) => {}
                                                    Effect::PlayCard => {}
                                                    Effect::PlusPower(x) => {}
                                                    Effect::PlusPowerForBattle(x) => {}
                                                    Effect::Rush => {}
                                                    _ => unreachable!(), // shouldn't have another TimedEffect inside the TimedEffect.
                                                }
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                _ => {}
                            }
                        }
                        if current_player_area.character.len()
                            == MAX_CHARACTER_AREA as usize
                        {
                            current_player_client
                                .send_message(ServerMessage::DiscardCharacter)
                                .await;
                            let discarded_character =
                                current_player_client.receive_next_nonidle_action().await;
                            match discarded_character {
                                PlayerAction::DiscardCharacter(i) => {
                                    let discarded_card =
                                        current_player_area.character.remove(i);
                                    current_player_area.player.trash.push(discarded_card);
                                }
                                _ => unreachable!(),
                            }
                        }
                        current_player_area.character.push(card);
                    }
                    _ => unreachable!(),
                }
            }
            PlayerAction::MainActivateCardEffect(c) => {
                // first, are we activating the leader or a character card?
                // then, can you pay for it?
            }
            PlayerAction::NoAction => {
                return (current_player_area, other_player_area, TurnPhase::Main);
            }
            _ => {
                panic!("I don't know how to handle this action yet.")
            }
        }

        (current_player_area, other_player_area, TurnPhase::Main)
    }

    pub async fn battle_attack_step() {}

    pub async fn battle_block_step() {}

    pub async fn battle_counter_step() {}

    pub async fn battle_damage_step() {}

    pub async fn battle_end() {}

    pub async fn end_step() {}
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum MainPhaseAction {
    PlayCard,
    ActivateCardEffect,
    AttachDon,
    Battle,
}

pub const MAX_CHARACTER_AREA: i32 = 5;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Timing {
    OnPlay,
    WhenAttacking,
    ActivateMain,
    Main, // basically ActivateMain, but for event cards.
    Counter,
    DuringTurn,
    Trigger,
    Always,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Effect {
    Blocker,
    Draw(i32),
    GiveOtherCardPower(i32),
    GiveRestedDon(i32),
    KnockOutWithPowerEqualOrLessThan(i32),
    OncePerTurn,
    OpponentNoBlocker(Condition),
    PlayCard,
    PlusPower(i32),
    PlusPowerForBattle(i32),
    Rush,
    TimedEffect(Timing, EffectCost, Vec<Effect>),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Condition {
    None,
    PowerAndAbove(i32),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum EffectCost {
    MinusDon(i32),
    RestDon(i32),
    DonAttached(i32),
    Zero, // Needed for timed effects that don't require a cost, makes more sense than doing `Option<EffectCost>` everywhere.
}
