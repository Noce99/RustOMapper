mod map_file;
mod map;
mod websocket_server;

use crate::map_file::{MapFile, map_finder::get_map_paths};
use crate::websocket_server::WebSocketServer;

#[macro_use] extern crate prettytable;

fn main(){
    let maps : Vec<MapFile> = get_map_paths();
    
    let mut server = WebSocketServer::new(maps);
    server.run();
}