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
use futures::prelude::*;
use serde_json::Value;
use tokio::net::TcpListener;
use tokio_serde::formats::*;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

use optcg::card::*;
use optcg::game::*;
use optcg::mockclient::*;
use optcg::player::*;
use optcg::*;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

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

    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            loop {
                let (rx, tx) = socket.split();
                let reader = FramedRead::new(rx, LengthDelimitedCodec::new());
                let writer = FramedWrite::new(tx, LengthDelimitedCodec::new());
                
                let mut deserial = tokio_serde::SymmetricallyFramed::new(
                    reader,
                    SymmetricalJson::<Value>::default(),
                );

                let mut serial = tokio_serde::SymmetricallyFramed::new(
                    writer,
                    SymmetricalJson::<Value>::default(),
                );
                
                let to_be_sent = serde_json::from_str(serde_json::to_string(&ServerMessage::Connected).unwrap().as_str()).unwrap();
                println!("SENDING: {:?}", to_be_sent);
                serial.send(to_be_sent).await.unwrap();
                tokio::time::sleep(std::time::Duration::from_millis(500)).await;

                while let Some(msg) = deserial.try_next().await.unwrap() {
                    println!("GOT: {:?}", msg);
                    let player_action = serde_json::from_value::<PlayerAction>(msg).unwrap();
                    println!("PARSED: {:?}", player_action);
                    execute_player_action(player_action).await.unwrap();
                    break;
                }
            }
        });
    }
} 

async fn execute_player_action(player_action: PlayerAction) -> std::io::Result<()> {
    use PlayerAction::*;
    match player_action {
        Idle => {}
        ReportDeck(_) => {}
        TakeMulligan => {}
        NoAction => {}
        MainActivateCardEffect(_) => {}
        MainPlayCard(_) => {}
        MainAttachDon(_) => {}
        MainBattle(_) => {}
        End => {}
    }
    
    Ok(())
}
