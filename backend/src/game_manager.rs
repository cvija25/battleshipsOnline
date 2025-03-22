use serde::{Deserialize, Serialize};
use tokio::sync::{broadcast, mpsc, Notify};
use std::{sync::Arc, usize};
use crate::utils::BiDirectionalChannel;

#[derive(Serialize, Deserialize, Debug)]
struct Cell {
    row: usize,
    col: usize,
}

// GameManager waits for two players before allowing them to interact
pub async fn game_manager(
    to_hc: BiDirectionalChannel,
    from_hc: BiDirectionalChannel
) {
    let player1 = from_hc.receive().await.unwrap();
    println!("Player joined: {}", player1);
    let player2 = from_hc.receive().await.unwrap();
    println!("Player joined: {}", player2);
    // game_ready_notify.notify_waiters();

    // let mut inner_board1:[[bool; 5];5] = [[false; 5];5];
    // let mut outter_borad1:[[bool; 5];5] = [[false; 5];5];

    // let mut inner_board2:[[bool; 5];5] = [[false; 5];5];
    // let mut outter_borad2:[[bool; 5];5] = [[false; 5];5];

    // // player1 board
    // if let Some(result) = rx.recv().await {
    //     let data: Vec<Cell> = serde_json::from_str(&result).unwrap();
    //     for (x, y) in data.iter().map(|el| (el.row, el.col)) {
    //         inner_board1[x][y] = true;
    //     }
    // }

    // // player2 board
    // if let Some(result) = rx.recv().await {
    //     let data: Vec<Cell> = serde_json::from_str(&result).unwrap();
    //     for (x, y) in data.iter().map(|el| (el.row, el.col)) {
    //         inner_board2[x][y] = true;
    //     }
    // }

    // game_ready_notify.notify_waiters();

    // println!("board1 {:?} \nboard2 {:?}", inner_board1, inner_board2);
    
    // let mut turn = 0;
    // while let Some(result) = rx.recv().await {
    //     let data: Cell = serde_json::from_str(&result).unwrap();
    //     if turn % 2 == 0 {
    //         if inner_board2[data.row][data.col] {
    //             to_client.send("Hit!".to_string()).expect("nije");
    //         } else {
    //             to_client.send("Miss!".to_string()).expect("nije");
    //         }
    //     } else {
    //         if inner_board1[data.row][data.col] {
    //             to_client.send("Hit!".to_string()).expect("nije");
    //         } else {
    //             to_client.send("Miss!".to_string()).expect("nije");
    //         }
    //     }
    //     turn += 1;
    // }
}