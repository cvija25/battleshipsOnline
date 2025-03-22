use rand;
use serde::Serialize;
use std::{collections::HashMap, sync::{Arc,Mutex}};
use crate::utils::BiDirectionalChannel;
use crate::game_manager::game_manager;

#[derive(Serialize)]
struct GameObj {
    p1:String,
    p2:String,
    game_id:String,
}

pub async fn matchmaker(to_hc: BiDirectionalChannel, from_hc: BiDirectionalChannel, games_map: Arc<Mutex<HashMap<String, (BiDirectionalChannel,BiDirectionalChannel)>>>) {
    let username = from_hc.receive().await.unwrap();
    println!("MM got username {}",username);
    let username2 = from_hc.receive().await.unwrap();
    println!("MM got username {}",username2);

    let game_id = rand::random_range(0..=10).to_string();
    let game_obj: GameObj = GameObj {p1:username, p2:username2, game_id: game_id.clone()};

    let ch1 = BiDirectionalChannel::new();
    let ch2 = BiDirectionalChannel::new();

    games_map.lock().unwrap().insert(game_id.clone(), (ch1.clone(), ch2.clone()));
    // // activates the thread for game instace, adds it to hashmap and returns main a gameID
    
    tokio::spawn(game_manager(ch1.clone(), ch2.clone()));

    to_hc.send(serde_json::to_string(&game_obj).unwrap());
}

// #[derive(Serialize, Deserialize, Debug)]
// struct Cell {
//     row: usize,
//     col: usize,
// }

// GameManager waits for two players before allowing them to interact