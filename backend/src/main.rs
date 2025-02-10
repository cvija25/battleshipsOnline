use serde::Deserialize;
use warp::filters::log::log;
use warp::{Reply,Rejection,http::StatusCode,http::Method};
use warp::Filter;
use warp::cors;
use warp::ws::{WebSocket,Message};
use futures::{SinkExt, StreamExt};
use tokio::sync::{mpsc,Notify, broadcast};
use std::sync::Arc;
use sqlx::{PgPool, Pool, Postgres, Row};
use std::convert::Infallible;
use auth::create_jwt;

mod game_manager;
mod auth;
#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String
}

#[derive(Deserialize)]
struct JWTreq {
    jwt: String
}

#[tokio::main]
async fn main() {
    let game_ready_notify = Arc::new(Notify::new());

    let (game_manager_tx, game_manager_rx) = mpsc::channel(2);
    let (gm_to_client_tx, gm_to_client_rx) =  broadcast::channel(10);
    
    tokio::spawn(game_manager::game_manager(game_manager_rx, gm_to_client_tx, game_ready_notify.clone()));

    let game_ready_filter = warp::any().map(move || game_ready_notify.clone());
    let game_manager_tx_filter = warp::any().map(move || game_manager_tx.clone());
    let rx = Arc::new(gm_to_client_rx);

    // creates route filter
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .and(game_manager_tx_filter)
        .and(with_receiver(rx))
        .and(game_ready_filter)
        .map(|ws: warp::ws::Ws, game_manager_tx, gm_to_client_rx, game_ready_notify| {
            ws.on_upgrade(move |socket| handle_socket(socket, game_manager_tx, gm_to_client_rx, game_ready_notify))
        });

    let cors = warp::cors()
        .allow_origin("http://localhost:3000")
        .allow_methods(&[Method::GET, Method::POST])
        .allow_headers(vec!["Content-Type", "Authorization"]);

    let connection = PgPool::connect("postgresql://postgres:password@localhost:5432/battleships").await.unwrap();
    let login_route = warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(connection.clone()))
        .and_then(login)
        .with(cors);

    // let konacno : String = result.get("username");

    let routes = ws_route.or(login_route);
    // println!("{}",konacno);
    println!("WebSocket server running on ws://localhost:8000/ws");

    // runs server
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}

fn with_db(pool: Pool<Postgres>) -> impl Filter<Extract = (Pool<Postgres>,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

async fn login(req: LoginRequest, pool: PgPool) -> Result<impl Reply, Rejection> {
    let query = "SELECT username FROM users WHERE username = $1 AND password = $2";

    let result = sqlx::query(query)
        .bind(&req.username)
        .bind(&req.password)
        .fetch_optional(&pool)
        .await
        .map_err(|_| warp::reject::custom(MyCustomError))?;

    if let Some(row) = result {
        let username: String = row.get("username");
        let token = create_jwt(req.username, req.password).unwrap();
        Ok(warp::reply::json(&token))
    } else {
        let error_msg = warp::reply::json(&"Invalid credentials"); 
        Ok(error_msg)
    }
}

// Custom error handling
#[derive(Debug)]
struct MyCustomError;
impl warp::reject::Reject for MyCustomError {}

fn with_receiver(
    rx: Arc<broadcast::Receiver<String>>
) -> impl Filter<Extract = (broadcast::Receiver<String>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || {
        // Clone the Arc, then resubscribe from the reference
        let rx = rx.clone();
        rx.resubscribe()
    })
}

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
    game_manager_tx: mpsc::Sender<String>,
    mut gm_to_client_rx: broadcast::Receiver<String>,
    game_ready_notify: Arc<Notify>
) {
    println!("New WebSocket connection established");

    // transmission, receiver
    let (mut tx, mut rx) = ws.split();
    let player_name = format!("player_{}", rand::random::<u8>());
    game_manager_tx.send(player_name).await.expect("nije poslato");

    game_ready_notify.notified().await;              
    println!("player joins");

    // send board
    if let Some(result) = rx.next().await {
        if let Ok(msg) = result {
            game_manager_tx.send(msg.to_str().unwrap().to_string()).await.expect("nije poslo");
        }
    }
    
    // wait for boards
    game_ready_notify.notified().await;

    // play moves untill win TODO
    println!("Game can start");

    while let Some(result) = rx.next().await {
        if let Ok(msg) = result {
            game_manager_tx.send(msg.to_str().unwrap().to_string()).await.expect("nije poslo");
        }
        if let Ok(from_gm) = gm_to_client_rx.recv().await {
            //println!("{}",from_gm);
            tx.send(Message::text(from_gm)).await.expect("msg");
        }
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
