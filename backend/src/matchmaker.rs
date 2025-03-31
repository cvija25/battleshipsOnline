use rand;
use serde::Serialize;
use std::{collections::HashMap, sync::{Arc,Mutex}};
// use crate::utils::BroadcastChannel;
use std::collections::VecDeque;
use crate::game_manager::game_manager;
use tokio::sync::{oneshot,broadcast,mpsc};

#[derive(Serialize)]
struct GameObj {
    p1:String,
    p2:String,
    game_id:String,
}

type GamesMapType = Arc<Mutex<HashMap<String, (mpsc::Sender<String>,broadcast::Receiver<String>)>>>;

async fn create_game(p1:String, p2:String, games_map: GamesMapType, to_hc: broadcast::Sender<String>) {
    let game_id = rand::random_range(0..=10).to_string();
    let game_obj: GameObj = GameObj {p1, p2, game_id};

    let (btx, _) = broadcast::channel(10);
    let (mtx, mrx) = mpsc::channel(10);

    games_map.lock().unwrap().insert(game_obj.game_id.clone(), (mtx, btx.subscribe()));
    
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

pub async fn matchmaker(to_hc: broadcast::Sender<String>, mut from_hc: mpsc::Receiver<String>, games_map: GamesMapType) {
    let mut queue = VecDeque::<String>::new();

    while let Some(username) = from_hc.recv().await {
        queue.push_back(username);
        if queue.len() == 2 {
            let p1 = queue.pop_front().unwrap();
            let p2 = queue.pop_front().unwrap();
            tokio::spawn(create_game(p1,p2,games_map.clone(), to_hc.clone()));
        }
    }
}