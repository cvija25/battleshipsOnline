use actix::prelude::*;
use std::collections::HashMap;
use tokio::sync::{mpsc,broadcast};
pub struct GetTX {
    pub game_id:String
}

pub struct GetRX {
    pub game_id:String
}

pub struct SetGame {
    pub key:String,
    pub value:(mpsc::Sender<String>,broadcast::Receiver<String>)
}

impl Message for SetGame {
    type Result = ();
}

impl Message for GetTX {
    type Result = mpsc::Sender<String>;
}

impl Message for GetRX {
    type Result = broadcast::Receiver<String>;
}

pub struct Games {
    pub games_map: HashMap<String, (mpsc::Sender<String>,broadcast::Receiver<String>)>
}

impl Actor for Games {
    type Context = Context<Self>;
}

impl Handler<GetTX> for Games {
    type Result = Response<mpsc::Sender<String>>;
    fn handle(&mut self, msg: GetTX, _ctx: &mut Self::Context) -> Self::Result {
        Response::reply(self.games_map[&msg.game_id].0.clone())
    }
}

impl Handler<GetRX> for Games {
    type Result =  Response<broadcast::Receiver<String>>;
    fn handle(&mut self, msg: GetRX, _ctx: &mut Self::Context) -> Self::Result {
        Response::reply(self.games_map[&msg.game_id].1.resubscribe())
    }
}

impl Handler<SetGame> for Games {
    type Result = ();
    fn handle(&mut self, msg: SetGame, _ctx: &mut Self::Context) -> Self::Result {
        self.games_map.insert(msg.key, msg.value);
    }
}