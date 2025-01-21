#![allow(unused)]

use std::sync::mpsc::{channel, Receiver, Sender};

use futures::prelude::*;
use log::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::net::tcp::{ReadHalf, WriteHalf};
use tokio_serde::formats::*;
use tokio_serde::Framed;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use super::{card::*, player::*, *};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Turn {
    P1,
    P2,
}

pub type PlayerId = Turn;

pub struct GameServer<'stream> {
    pub game: PlayField,
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
        let mut count = 0;
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
pub struct PlayField {
    pub turn: Turn,
    pub turn_phase: TurnPhase,
    pub turn_n: i32,
    pub player_1: Box<Player>,
    pub player_2: Box<Player>,
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

    pub fn from_playfield(playfield: &PlayField) -> PublicPlayfieldState {
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

impl PlayField {
    pub async fn setup<'stream>(
        mut player_1: Player,
        mut player_2: Player,
        mut p1_client: &mut PlayerClient<'stream>,
        mut p2_client: &mut PlayerClient<'stream>,
    ) -> PlayField {
        let mut rng = StdRng::seed_from_u64(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u64,
        );

        player_1.shuffle(&mut rng);
        player_2.shuffle(&mut rng);

        player_1.draw(5).unwrap();
        player_2.draw(5).unwrap();

        let mut player_1 = Box::new(player_1);
        let mut player_2 = Box::new(player_2);
        let public_playfield_state = Box::new(PublicPlayfieldState::empty());

        p1_client.send_message(ServerMessage::PlayerDataPayload(player_1.clone())).await;
        p2_client.send_message(ServerMessage::PlayerDataPayload(player_2.clone())).await;
        p1_client.send_message(ServerMessage::OtherPlayerDataPayload(player_2.public_clone())).await;
        p2_client.send_message(ServerMessage::OtherPlayerDataPayload(player_1.public_clone())).await;
        p1_client.send_message(ServerMessage::PublicPlayfieldStateDataPayload(Box::new(PublicPlayfieldState::empty()))).await;
        p2_client.send_message(ServerMessage::PublicPlayfieldStateDataPayload(Box::new(PublicPlayfieldState::empty()))).await;

        // Query Mulligan
        p1_client.send_message(ServerMessage::QueryMulligan).await;
        let p1_mulligan = p1_client.receive_next_nonidle_action().await;

        if let PlayerAction::TakeMulligan = p1_mulligan {
            player_1.topdeck_hand();
            player_1.shuffle(&mut rng);
            player_1.draw(5).unwrap();
            p1_client
                .send_message(ServerMessage::PlayerDataPayload(player_1.clone()))
                .await;
        }

        p2_client.send_message(ServerMessage::QueryMulligan).await;
        let p2_mulligan = p2_client.receive_next_nonidle_action().await;

        if let PlayerAction::TakeMulligan = p2_mulligan {
            player_2.topdeck_hand();
            player_2.shuffle(&mut rng);
            player_2.draw(5).unwrap();
            p2_client
                .send_message(ServerMessage::PlayerDataPayload(player_2.clone()))
                .await;
        }

        let p1_life = player_1.draw_out(player_1.leader.life()).unwrap();
        let p2_life = player_2.draw_out(player_2.leader.life()).unwrap();

        // Begin turn 1.
        let p1_don = player_1.draw_don(1);
        
        p1_client.send_message(ServerMessage::PlayerDataPayload(player_1.clone())).await;
        p2_client.send_message(ServerMessage::PlayerDataPayload(player_2.clone())).await;
        p1_client.send_message(ServerMessage::OtherPlayerDataPayload(player_2.public_clone())).await;
        p2_client.send_message(ServerMessage::OtherPlayerDataPayload(player_1.public_clone())).await;

        PlayField {
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

    pub fn trigger_loser(&self) -> Option<PlayerId> {
        todo!()
    }

    pub async fn step<'stream>(
        &mut self,
        mut p1_client: &mut PlayerClient<'stream>,
        mut p2_client: &mut PlayerClient<'stream>,
    ) {
        use TurnPhase::*;

        debug!("{:?}, {:?}, {:?}", self.turn, self.turn_phase, self.turn_n);

        let (current_player, other_player) = match self.turn {
            Turn::P1 => (&mut self.player_1, &mut self.player_2),
            Turn::P2 => (&mut self.player_2, &mut self.player_1),
        };

        let (current_player_client, other_player_client) = match self.turn {
            Turn::P1 => (p1_client, p2_client),
            Turn::P2 => (p2_client, p1_client),
        };

        async fn send_updates<'stream>(
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

        // Behold! A state machine!
        match self.turn_phase {
            Refresh => {
                debug!("(TURN) [REFRESH]");
                self.p1_active_don_area.append(&mut self.p1_rested_don_area);
                self.p1_character_area
                    .append(&mut self.p1_rested_character_area);
                for card in self.p1_character_area.iter_mut() {
                    self.p1_active_don_area.append(&mut card.attached_don);
                }

                self.p2_active_don_area.append(&mut self.p2_rested_don_area);
                self.p2_character_area
                    .append(&mut self.p2_rested_character_area);
                for card in self.p2_character_area.iter_mut() {
                    self.p2_active_don_area.append(&mut card.attached_don);
                }

                let public_state = PublicPlayfieldState::new(
                    self.p1_life_area.clone(),
                    self.p2_life_area.clone(),
                    self.p1_stage_area.clone(),
                    self.p2_stage_area.clone(),
                    self.p1_character_area.clone(),
                    self.p2_character_area.clone(),
                    self.p1_rested_character_area.clone(),
                    self.p2_rested_character_area.clone(),
                    self.p1_active_don_area.clone(),
                    self.p2_active_don_area.clone(),
                    self.p1_rested_don_area.clone(),
                    self.p2_rested_don_area.clone(),
                );
                send_updates(
                    current_player_client,
                    other_player_client,
                    current_player,
                    other_player,
                    public_state,
                );

                self.turn_phase = Draw;
            }
            Draw => {
                debug!("(TURN) [DRAW]");
                let res = current_player.draw(1);
                match res {
                    Ok(()) => {}
                    Err(()) => {
                        return;
                    }
                }
                let public_state = PublicPlayfieldState::new(
                    self.p1_life_area.clone(),
                    self.p2_life_area.clone(),
                    self.p1_stage_area.clone(),
                    self.p2_stage_area.clone(),
                    self.p1_character_area.clone(),
                    self.p2_character_area.clone(),
                    self.p1_rested_character_area.clone(),
                    self.p2_rested_character_area.clone(),
                    self.p1_active_don_area.clone(),
                    self.p2_active_don_area.clone(),
                    self.p1_rested_don_area.clone(),
                    self.p2_rested_don_area.clone(),
                );
                send_updates(
                    current_player_client,
                    other_player_client,
                    current_player,
                    other_player,
                    public_state,
                ).await;
                self.turn_phase = Don;
            }
            Don => {
                debug!("(TURN) [DON]");
                current_player.draw_don(2);

                let public_state = PublicPlayfieldState::new(
                    self.p1_life_area.clone(),
                    self.p2_life_area.clone(),
                    self.p1_stage_area.clone(),
                    self.p2_stage_area.clone(),
                    self.p1_character_area.clone(),
                    self.p2_character_area.clone(),
                    self.p1_rested_character_area.clone(),
                    self.p2_rested_character_area.clone(),
                    self.p1_active_don_area.clone(),
                    self.p2_active_don_area.clone(),
                    self.p1_rested_don_area.clone(),
                    self.p2_rested_don_area.clone(),
                );
                send_updates(
                    current_player_client,
                    other_player_client,
                    current_player,
                    other_player,
                    public_state,
                ).await;

                self.turn_phase = Main;
            }
            Main => {
                debug!("(TURN) [MAIN]");
                let public_state = PublicPlayfieldState::new(
                    self.p1_life_area.clone(),
                    self.p2_life_area.clone(),
                    self.p1_stage_area.clone(),
                    self.p2_stage_area.clone(),
                    self.p1_character_area.clone(),
                    self.p2_character_area.clone(),
                    self.p1_rested_character_area.clone(),
                    self.p2_rested_character_area.clone(),
                    self.p1_active_don_area.clone(),
                    self.p2_active_don_area.clone(),
                    self.p1_rested_don_area.clone(),
                    self.p2_rested_don_area.clone(),
                );
                send_updates(
                    current_player_client,
                    other_player_client,
                    current_player,
                    other_player,
                    public_state,
                ).await;

                debug!("Sending {:?} to {}", ServerMessage::TakeMainAction, current_player.name);
                current_player_client.send_message(ServerMessage::TakeMainAction).await;
                let player_action = current_player_client.receive_next_nonidle_action().await;
                debug!("Received {:?}", player_action);
                match player_action {
                    PlayerAction::End => {
                        self.turn_phase = End;
                        return;
                    }
                    _ => {}
                }
            }
            BattleAttackStep => {}
            BattleBlockStep => {}
            BattleCounterStep => {}
            BattleDamageStep => {}
            BattleEnd => {}
            End => {
                debug!("(TURN) [END]");
                self.turn_n += 1;
                self.turn = match self.turn {
                    Turn::P1 => Turn::P2,
                    Turn::P2 => Turn::P1,
                };
                self.turn_phase = Refresh;
            }
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
