use serde::{Deserialize, Serialize};
use tokio::sync::{oneshot, broadcast, mpsc};
use std::usize;

#[derive(Serialize, Deserialize, Debug)]
struct Cell {
    row: usize,
    col: usize,
}

// GameManager waits for two players before allowing them to interact
pub async fn game_manager(
    to_hc: broadcast::Sender<String>,
    mut from_hc: mpsc::Receiver<String>,
    to_mm : oneshot::Sender<usize>,
) {
    to_mm.send(1).expect("error sending ready to mm");
    println!("GM waiting for player");
    let player1 = from_hc.recv().await.expect("error getting username from hc");
    println!("GM Player joined: {}", player1);
    let player2 = from_hc.recv().await.expect("error getting username from hc");
    println!("GM Player joined: {}", player2);

    // to_hc.send("pocinjemo".to_string()).expect("error sending ready from gm to hc");

    let mut inner_board1:[[bool; 5];5] = [[false; 5];5];
    // let mut outter_borad1:[[bool; 5];5] = [[false; 5];5];

    let mut inner_board2:[[bool; 5];5] = [[false; 5];5];
    // let mut outter_borad2:[[bool; 5];5] = [[false; 5];5];

    // player1 board
    if let Some(result) = from_hc.recv().await {

        let data: Vec<Cell> = serde_json::from_str(&result).unwrap();
        for (x, y) in data.iter().map(|el| (el.row, el.col)) {
            inner_board1[x][y] = true;
        }
    }

    // player2 board
    if let Some(result) = from_hc.recv().await {
        let data: Vec<Cell> = serde_json::from_str(&result).unwrap();
        for (x, y) in data.iter().map(|el| (el.row, el.col)) {
            inner_board2[x][y] = true;
        }
    }

    println!("board1 {:?} \nboard2 {:?}", inner_board1, inner_board2);
    
    to_hc.send(player1.clone()).expect("cant sent turn to hc");
    let mut turn = 0;
    while let Some(result) = from_hc.recv().await {
        let data: Cell = serde_json::from_str(&result).unwrap();
        if turn % 2 == 0 {
            if inner_board2[data.row][data.col] {
                to_hc.send("Hit!".to_string()).expect("nije");
            } else {
                to_hc.send("Miss!".to_string()).expect("nije");
            }
            to_hc.send(player2.clone());
        } else {
            if inner_board1[data.row][data.col] {
                to_hc.send("Hit!".to_string()).expect("nije");
            } else {
                to_hc.send("Miss!".to_string()).expect("nije");
            }
            to_hc.send(player1.clone());
        }
        turn += 1;
    }
}