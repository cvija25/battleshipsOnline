use std::{collections::HashMap, sync::{Arc,Mutex}};

//use futures::lock::Mutex;
use serde::{Deserialize,Serialize};
// use warp::{Reply,Rejection,http::Method};
use warp::Filter;
// use warp::cors;
use warp::ws::{WebSocket,Message};
use futures::{SinkExt, StreamExt};
// use sqlx::{PgPool, Pool, Postgres, Row};
// use std::convert::Infallible;
// use auth::create_jwt;
use tokio::sync::{broadcast,mpsc};

mod game_manager;
mod auth;
mod matchmaker;
mod utils;

// use utils::BroadcastChannel;

// #[derive(Deserialize)]
// struct LoginRequest {
//     username: String,
//     password: String
// }

// #[derive(Deserialize)]
// struct GameRequest {
//     username: String
// }

#[derive(Serialize, Deserialize)]
struct GameObj {
    p1:String,
    p2:String,
    game_id:String,
}

// #[derive(Deserialize)]
// struct JWTreq {
//     jwt: String
// }


#[tokio::main]
async fn main() {

    let (btx,_) = broadcast::channel(10);
    let (mtx,mrx) = mpsc::channel(10);
    
    // key: gameID | value: (to_x, from_x)
    let games_map: Arc<Mutex<HashMap<String, (mpsc::Sender<String>,broadcast::Receiver<String>)>>> = Arc::new(Mutex::new(HashMap::new()));

    tokio::spawn(matchmaker::matchmaker(btx.clone(), mrx, games_map.clone()));

    let channel1_filter = warp::any().map(move || mtx.clone());
    let channel2_filter = warp::any().map(move || btx.subscribe());
    let games_map_filter = warp::any().map(move || games_map.clone());

    // routes
    // --------------------------------------------------------
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(channel1_filter)
        .and(channel2_filter)
        .and(games_map_filter)
        .map(|ws: warp::ws::Ws, to_mm: mpsc::Sender<String>, from_mm: broadcast::Receiver<String>, games_map: Arc<Mutex<HashMap<String, (mpsc::Sender<String>,broadcast::Receiver<String>)>>>| {
            ws.on_upgrade(move |socket| handle_socket(socket,to_mm, from_mm, games_map))
        });

    // let game_route = warp::path("game")
    //     .and(warp::post())
    //     .and(warp::body::json())
    //     .and_then(game_req_handler)
    //     .with(cors);

    // let cors = warp::cors()
    //     .allow_origin("http://localhost:3000")
    //     .allow_methods(&[Method::GET, Method::POST])
    //     .allow_headers(vec!["Content-Type", "Authorization"]);

    // let connection = PgPool::connect("postgresql://postgres:password@localhost:5432/battleships").await.unwrap();
    // let login_route = warp::path("login")
    //     .and(warp::post())
    //     .and(warp::body::json())
    //     .and(with_db(connection.clone()))
    //     .and_then(login)
    //     .with(cors);

    // let konacno : String = result.get("username");

    let routes = ws_route;//.or(login_route);
    // println!("{}",konacno);
    println!("WebSocket server running on ws://localhost:8000/ws");

    // runs server
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

// fn with_db(pool: Pool<Postgres>) -> impl Filter<Extract = (Pool<Postgres>,), Error = Infallible> + Clone {
//     warp::any().map(move || pool.clone())
// }

// async fn game_req_handler(req: GameRequest) -> Result<impl Reply, Rejection> {}

// async fn login(req: LoginRequest, pool: PgPool) -> Result<impl Reply, Rejection> {
//     let query = "SELECT username FROM users WHERE username = $1 AND password = $2";

//     let result = sqlx::query(query)
//         .bind(&req.username)
//         .bind(&req.password)
//         .fetch_optional(&pool)
//         .await
//         .map_err(|_| warp::reject::custom(MyCustomError))?;

//     if let Some(row) = result {
//         let username: String = row.get("username");
//         let token = create_jwt(req.username, req.password).unwrap();
//         Ok(warp::reply::json(&token))
//     } else {
//         let error_msg = warp::reply::json(&"Invalid credentials"); 
//         Ok(error_msg)
//     }
// }

// Custom error handling
// #[derive(Debug)]
// struct MyCustomError;
// impl warp::reject::Reject for MyCustomError {}

// fn with_receiver(
//     rx: Arc<broadcast::Receiver<String>>
// ) -> impl Filter<Extract = (broadcast::Receiver<String>,), Error = std::convert::Infallible> + Clone {
//     warp::any().map(move || {
//         // Clone the Arc, then resubscribe from the reference
//         let rx = rx.clone();
//         rx.resubscribe()
//     })
// }

    /*
        {
            "rows":5,
            "ships":{
                    "Ship1" : {
                        "cells" : [
                            {"x":0,"y":0},
                            {"x":0,"y":1},
                            {"x":0,"y":2}
                        ],
                        "health" : 3
                    },
                    "Ship2" : {
                        "cells" : [
                            {"x":1,"y":0},
                            {"x":2,"y":0}
                        ],
                        "health" : 2
                    }
                }           
        }
    */

// Handle the WebSocket connection
async fn handle_socket(
    ws: WebSocket,
    to_mm: mpsc::Sender<String>,
    mut from_mm: broadcast::Receiver<String>,
    games_map: Arc<Mutex<HashMap<String, (mpsc::Sender<String>,broadcast::Receiver<String>)>>>
) {
    println!("New WebSocket connection established");

    // transmission, receiver
    let (mut tx, mut rx) = ws.split();
    let player_name = format!("player_{}", rand::random::<u8>());
    
    to_mm.send(player_name.clone()).await.expect("hc cant send to mm");
    
    let game_obj_str: String = from_mm.recv().await.expect("error receiving gameobj from mm");
    println!("HC game_obj_str {} for {}",game_obj_str,player_name);

    let game_obj : GameObj = serde_json::from_str(&game_obj_str).expect("error parsing gameobj"); 
    let game_id : String = game_obj.game_id;
    // // gets game chan from gameinstacemap

    let (to_gm, from_gm) =  {
        let lock = games_map.lock().unwrap();
        // let map = &*lock;
        let (a,b) = lock.get(&game_id).unwrap();
        (a.clone(), b.resubscribe())
    };

    println!("HC sending to gm");
    to_gm.send(player_name.clone()).await.expect("hc can't send to gm");
    tx.send(Message::text(game_id)).await.expect("error sending gameID to client");

    println!("HC Receiving board from client");
    if let Some(res1)= rx.next().await {
        if let Ok(res2) = res1 {
            if let Ok(res3)= res2.to_str() {
                println!("Sending board to gm");
                to_gm.send(res3.to_string()).await.expect("cant send board to gm");
            } else {
                println!("got error parsing message to string");
            }
            
        } else {
            println!("got errror from client");
        }
    }
    let mut from_gm1 = from_gm.resubscribe();
    let mut turn = from_gm1.recv().await.expect("error receiveing turn from gm");
    let mut result = String::new();
    rx.next().await; // first client ready
    rx.next().await; // second client ready
    println!("turn: {}", turn);
    tx.send(Message::text("gameLoad")).await.expect("error sending gameLoad to client");
    loop {
        if turn == player_name {
            tx.send(Message::text("ready")).await.expect("error sending redy to client");
            let p_move = rx.next().await.unwrap().unwrap().to_str().unwrap().to_string();
            to_gm.send(p_move).await.expect("error sending move to gm");
        } else {
            tx.send(Message::text("no")).await.expect("error sending unready to client");
        }
        result = from_gm1.recv().await.expect("error receiving result");
        tx.send(Message::text(result.clone())).await.expect("error sending res to client");
        if result == "win" || result == "tie" {
            break;
        }
        turn = from_gm1.recv().await.expect("error receiveing turn from gm");
    }
    println!("WebSocket connection closed");
}

/*
TODO:
Player:
    boats -> {"x":2, "y":3, "z":4}
    board -> [
              [x],[x],[y],[o],
              [o],[o],[y],[o],
              [o],[o],[y],[o]
              [z],[z],[z],[z]
            ]

*/
