use rand;
use serde::Serialize;
use std::{collections::HashMap, sync::{Arc,Mutex}};
// use crate::utils::BroadcastChannel;
use crate::game_manager::game_manager;
use tokio::sync::{oneshot,broadcast,mpsc};

#[derive(Serialize)]
struct GameObj {
    p1:String,
    p2:String,
    game_id:String,
}

pub async fn matchmaker(to_hc: broadcast::Sender<String>, mut from_hc: mpsc::Receiver<String>, games_map: Arc<Mutex<HashMap<String, (mpsc::Sender<String>,broadcast::Receiver<String>)>>>) {
    let username = from_hc.recv().await.expect("error getting username from hc");
    println!("MM got username {}",username);
    let username2 = from_hc.recv().await.expect("error getting username from hc");
    println!("MM got username {}",username2);

    let game_id = rand::random_range(0..=10).to_string();
    let game_obj: GameObj = GameObj {p1:username, p2:username2, game_id: game_id.clone()};

    let (btx, _) = broadcast::channel(10);
    let (mtx, mrx) = mpsc::channel(10);

    games_map.lock().unwrap().insert(game_id.clone(), (mtx, btx.subscribe()));
    
    println!("MM spawned gm");
    let (tx,rx) = oneshot::channel();
    // activates the thread for game instace, adds it to hashmap and returns main a gameID
    tokio::spawn(game_manager(btx, mrx, tx));
    match rx.await {
        Ok(_) => println!("gm is ready"),
        Err(_) => println!("error from gm to mm"),
    }

    println!("MM send to_hc");
    to_hc.send(serde_json::to_string(&game_obj).unwrap()).expect("error sending gameObj to hc");
}