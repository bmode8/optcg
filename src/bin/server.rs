#![allow(unused)]
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{stdin, Read, Write};
use std::sync::mpsc::{channel, Receiver, Sender};

use futures::prelude::*;
use log::*;
use rand::prelude::*;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use simplelog::*;
use tokio::net::TcpListener;
use tokio::*;
use tokio_serde::formats::*;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use optcg::card::*;
use optcg::game::*;
use optcg::mockclient::*;
use optcg::player::*;
use optcg::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
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

    let player_1_default = Player {
        name: "Player 1".into(),
        leader: leader.clone(),
        main_deck: main_deck.clone(),
        don_deck: don_deck.clone(),
        hand: vec![],
        trash: vec![],
    };

    let player_2_default = Player {
        name: "Player 2".into(),
        leader: leader.clone(),
        main_deck: main_deck.clone(),
        don_deck: don_deck.clone(),
        hand: vec![],
        trash: vec![],
    };
    
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (mut p1_socket, _) = listener.accept().await?;
        let (mut p2_socket, _) = listener.accept().await?;

        let player_1 = player_1_default.clone();
        let player_2 = player_2_default.clone();

        tokio::spawn(async move {
            let (p1_rx, p1_tx) = p1_socket.split();
            let p1_read = FramedRead::new(p1_rx, LengthDelimitedCodec::new());
            let p1_write = FramedWrite::new(p1_tx, LengthDelimitedCodec::new());

            let mut p1_reader =
                tokio_serde::SymmetricallyFramed::new(p1_read, SymmetricalJson::<Value>::default());

            let mut p1_writer = tokio_serde::SymmetricallyFramed::new(
                p1_write,
                SymmetricalJson::<Value>::default(),
            );

            let mut p1_client = PlayerClient {
                player: Box::new(player_1.clone()),
                reader: p1_reader,
                writer: p1_writer,
            };

            let (p2_rx, p2_tx) = p2_socket.split();
            let p2_read = FramedRead::new(p2_rx, LengthDelimitedCodec::new());
            let p2_write = FramedWrite::new(p2_tx, LengthDelimitedCodec::new());

            let mut p2_reader =
                tokio_serde::SymmetricallyFramed::new(p2_read, SymmetricalJson::<Value>::default());

            let mut p2_writer = tokio_serde::SymmetricallyFramed::new(
                p2_write,
                SymmetricalJson::<Value>::default(),
            );

            let mut p2_client = PlayerClient {
                player: Box::new(player_2.clone()),
                reader: p2_reader,
                writer: p2_writer,
            };

            let mut playfield = PlayField::setup(
                player_1.clone(),
                player_2.clone(),
                &mut p1_client,
                &mut p2_client,
            ).await;

            // Main loop for a game.
            loop {
                p1_client.send_message(ServerMessage::Connected).await;
                p2_client.send_message(ServerMessage::Connected).await;
                
                let winner: Option<Turn> = playfield.check_loser(); // `Turn` is a unique representation of each player, so it works best here.
                if let Some(winner) = winner {
                    println!("Player {:?} wins!", winner);
                    break;
                }

                playfield.step(&mut p1_client, &mut p2_client).await;

                while let Some(next) = p1_client.reader.try_next().await.unwrap() {
                    let message = serde_json::from_value::<PlayerAction>(next).unwrap();
                    execute_player_action(message).await.unwrap();
                    break;
                }

                while let Some(next) = p2_client.reader.try_next().await.unwrap() {
                    let message = serde_json::from_value::<PlayerAction>(next).unwrap(); 
                    execute_player_action(message).await.unwrap();
                    break;
                }
                
                
                //tokio::time::sleep(std::time::Duration::from_millis(500)).await;
            }
        });
    }
}

async fn execute_player_action(player_action: PlayerAction) -> std::io::Result<()> {
    use PlayerAction::*;
    match player_action {
        Idle => {
            debug!("Idle");
            return Ok(())
        }
        ReportDeck(_) => {}
        TakeMulligan => {}
        NoAction => {}
        MainActivateCardEffect(_) => {}
        MainPlayCard(_) => {}
        MainAttachDon(_) => {}
        MainBattle(_) => {}
        End => { 
            debug!("End");
            return Ok(())
        }
    }

    Ok(())
}
