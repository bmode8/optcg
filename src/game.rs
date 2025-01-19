#![allow(unused)]

use std::sync::mpsc::{channel, Receiver, Sender};

use log::*;
use rand::prelude::*;
use serde::{Deserialize, Serialize};

use super::{card::*, mockclient::*, player::*, *};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Turn {
    P1,
    P2,
}

pub type PlayerId = Turn;

#[derive(Debug)]
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
        let public_playfield_state = Box::new(PublicPlayfieldState::empty());

        let mut player_1_client = MockPlayerClient {
            this_player: player_1.clone(),
            other_player: player_2.public_clone(),
            public_playfield_state: public_playfield_state.clone(),
            tx: p1_client_sender,
            rx: p1_server_receiver,
        };

        let mut player_2_client = MockPlayerClient {
            this_player: player_2.clone(),
            other_player: player_1.public_clone(),
            public_playfield_state: public_playfield_state.clone(),
            tx: p2_client_sender,
            rx: p2_server_receiver,
        };

        p1_sender.send(ServerMessage::QueryMulligan).unwrap();
        player_1_client.handle_messages();
        let p1_mulligan = p1_receiver.recv().unwrap();

        if let PlayerAction::TakeMulligan = p1_mulligan {
            player_1.topdeck_hand();
            player_1.shuffle(&mut rng);
            player_1.draw(5).unwrap();
            p1_sender
                .send(ServerMessage::PlayerDataPayload(player_1.clone()))
                .unwrap();
            player_1_client.handle_messages();
        }

        p2_sender.send(ServerMessage::QueryMulligan).unwrap();
        player_2_client.handle_messages();
        let p2_mulligan = p2_receiver.recv().unwrap();

        if let PlayerAction::TakeMulligan = p2_mulligan {
            player_2.topdeck_hand();
            player_2.shuffle(&mut rng);
            player_2.draw(5).unwrap();
            p2_sender
                .send(ServerMessage::PlayerDataPayload(player_2.clone()))
                .unwrap();
            player_2_client.handle_messages();
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
            return Some(PlayerId::P1);
        } else if p2_deck_len == 0 {
            return Some(PlayerId::P2);
        }

        None
    }

    pub fn trigger_loser(&self) -> Option<PlayerId> {
        todo!()
    }

    pub fn step(&mut self) {
        use TurnPhase::*;

        let (current_player, other_player) = match self.turn {
            Turn::P1 => (&mut self.player_1, &mut self.player_2),
            Turn::P2 => (&mut self.player_2, &mut self.player_1),
        };

        let (current_tx, current_rx, other_tx, other_rx) = match self.turn {
            Turn::P1 => (
                &mut self.p1_sender,
                &mut self.p1_receiver,
                &mut self.p2_sender,
                &mut self.p2_receiver,
            ),
            Turn::P2 => (
                &mut self.p2_sender,
                &mut self.p2_receiver,
                &mut self.p1_sender,
                &mut self.p1_receiver,
            ),
        };

        fn send_updates(
            current_tx: &mut Sender<ServerMessage>,
            other_tx: &mut Sender<ServerMessage>,
            current_player: &Box<Player>,
            other_player: &Box<Player>,
            public_field_state: PublicPlayfieldState,
        ) {
            current_tx
                .send(ServerMessage::PlayerDataPayload(current_player.clone()))
                .unwrap();
            current_tx
                .send(ServerMessage::OtherPlayerDataPayload(
                    other_player.public_clone(),
                ))
                .unwrap();
            current_tx
                .send(ServerMessage::PublicPlayfieldStateDataPayload(Box::new(
                    public_field_state.clone(),
                )))
                .unwrap();
            other_tx
                .send(ServerMessage::PlayerDataPayload(other_player.clone()))
                .unwrap();
            other_tx
                .send(ServerMessage::OtherPlayerDataPayload(
                    current_player.public_clone(),
                ))
                .unwrap();
            other_tx
                .send(ServerMessage::PublicPlayfieldStateDataPayload(Box::new(
                    public_field_state,
                )))
                .unwrap();
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
                    current_tx,
                    other_tx,
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
                    current_tx,
                    other_tx,
                    current_player,
                    other_player,
                    public_state,
                );
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
                    current_tx,
                    other_tx,
                    current_player,
                    other_player,
                    public_state,
                );

                self.turn_phase = Main;
            }
            Main => {
                debug!("(TURN) [MAIN]");
                let player_action = current_rx.try_recv();
                match player_action {
                    Ok(PlayerAction::End) => {
                        self.turn_phase = End;
                        return;
                    }
                    Ok(_) => {}
                    Err(_e) => {}
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
                    current_tx,
                    other_tx,
                    current_player,
                    other_player,
                    public_state,
                );

                current_tx.send(ServerMessage::TakeMainAction).unwrap();
            }
            BattleAttackStep => {}
            BattleBlockStep => {}
            BattleCounterStep => {}
            BattleDamageStep => {}
            BattleEnd => {}
            End => {
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
