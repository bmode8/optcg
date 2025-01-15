#![allow(unused)]
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{stdin, Read, Write};
use std::sync::mpsc::{channel, Receiver, Sender};

use log::*;
use rand::prelude::*;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use simplelog::*;
use tokio::*;

use optcg::card::*;
use optcg::game::*;
use optcg::mockclient::*;
use optcg::player::*;
use optcg::*;

fn main() {
    TermLogger::init(
        LevelFilter::Debug,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();
    install_card_data();

    let mut deck_list_file = File::open("sample_deck.txt").unwrap();
    let mut deck_list = String::new();
    deck_list_file.read_to_string(&mut deck_list).unwrap();

    let (leader, main_deck, don_deck) = parse_deck_list(deck_list.as_str()).unwrap();

    let player_1 = Player {
        name: "Player 1".into(),
        leader: leader.clone(),
        main_deck: main_deck.clone(),
        don_deck: don_deck.clone(),
        hand: vec![],
        trash: vec![],
    };

    let player_2 = Player {
        name: "Player 2".into(),
        leader: leader.clone(),
        main_deck: main_deck.clone(),
        don_deck: don_deck.clone(),
        hand: vec![],
        trash: vec![],
    };

    let (mut play_field, mut p1_client, mut p2_client) = PlayField::setup(player_1, player_2);

    loop {
        let winner: Option<Turn> = play_field.check_loser(); // `Turn` is a unique representation of each player, so it works best here.
        if let Some(winner) = winner {
            println!("Player {:?} wins!", winner);
            break;
        }

        play_field.step();
        debug!("{:?}'s TURN (TURN {})", play_field.turn, play_field.turn_n);
        match play_field.turn {
            Turn::P1 => {
                p1_client.handle_messages();
            }
            Turn::P2 => {
                p2_client.handle_messages();
            }
        }

        println!("{:?}", play_field);
    }
}
