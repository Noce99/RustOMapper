mod websocket_server;

use rust_o_mapper::map_file::{MapFile, map_finder::get_map_paths};
use crate::websocket_server::WebSocketServer;

fn main(){
    let maps : Vec<MapFile> = get_map_paths();
    
    let mut server = WebSocketServer::new(maps);
    server.run();
}