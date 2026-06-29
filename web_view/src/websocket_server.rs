use std::env;
use std::path::PathBuf;
use std::net::TcpListener;
use std::process::exit;
use std::string::ToString;
use tungstenite::accept;
use serde_json;
use serde::{Deserialize, Serialize};
use rust_o_mapper::map_file::MapFile;
use rust_o_mapper::map::yaml_encoding::map_to_yaml;

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    request: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PathsResponse {
    response_type: String,
    paths: Vec<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
struct UnknownRequestResponse {
    response_type: String,
    request: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ErrorResponse {
    response_type: String,
    error: String,
}

pub struct WebSocketServer{
    server: TcpListener,
    maps: Vec<MapFile>
}

impl WebSocketServer {
    pub fn new(maps: Vec<MapFile>) -> WebSocketServer {
        let server_result = TcpListener::bind("127.0.0.1:1999");
        let server;
        match server_result {
            Ok(listener) => {
                server = listener;
            },
            Err(error) => {
                eprintln!("Error While Creating the WebSocket Server:\n\t{}", error);
                exit(1);
            }
        }
        WebSocketServer{
            server,
            maps
        }
    }
    pub fn run(&mut self){
        println!("Listening on ws://127.0.0.1:1999");
        let mut current_dir = env::current_dir().expect("Failed to get current directory! WTF!");
        current_dir.push("web_src");
        current_dir.push("index.html");
        println!("Open: file://{}", current_dir.display());

        for stream_result in self.server.try_clone().unwrap().incoming() {
            let stream;
            match stream_result {
                Ok(a_stream) => { stream = a_stream; }
                Err(error) => {
                    eprintln!("Error in Incoming Connection, Skipping:\n\t{}", error);
                    continue;
                }
            }
            let websocket_result = accept(stream);
            let mut websocket;
            match websocket_result {
                Ok(a_websocket) => { websocket = a_websocket; }
                Err(error) => {
                    eprintln!("Error in WebSocket Handshake, Skipping:\n\t{}", error);
                    continue;
                }
            }
            let client_ip_result = websocket.get_ref().peer_addr();
            match client_ip_result {
                Ok(client_ip) => {
                    println!("New connection from {}", client_ip);
                }
                Err(error) => {
                    eprintln!("Error in getting Client Ip:\n\t{}", error);
                }
            }


            loop {
                match websocket.read() {
                    Ok(msg) => {
                        let msg_text = msg.to_text().unwrap();
                        println!("Received websocket request:\n\t{}", msg_text);

                        match self.parse_request(msg_text) {
                            Ok(response) => {
                                match websocket.send(response.into()) {
                                    Ok(_) => (),
                                    Err(e) => {
                                        println!("Error sending response, client disconnected?\n\t[{}]", e);
                                        break;
                                    },
                                }
                            },
                            Err(e) => {
                                eprintln!("Error parsing the websocket request!");
                                let error = ErrorResponse{
                                    response_type : "error".to_string(),
                                    error: e.to_string(),
                                };
                                let response = serde_json::to_string_pretty(&error).unwrap();
                                match websocket.send(response.into()) {
                                    Ok(_) => (),
                                    Err(e) => {
                                        println!("Error sending response, client disconnected?\n\t[{}]", e);
                                        break;
                                    },
                                }
                            },
                        }
                    }
                    Err(e) => {
                        println!("Client disconnected: {}", e);
                        break;
                    }
                }
            }
        }
    }

    fn parse_request(&mut self, json_str: &str) -> Result<String, Box<dyn std::error::Error>> {
        let incoming: Request = serde_json::from_str(json_str)?;

        let mut response;
        let a_response = ErrorResponse {
            response_type: "error".to_string(),
            error: "Not Initialized WTF! :-(".to_string(),
        };
        response = Ok(serde_json::to_string_pretty(&a_response).unwrap());
        match incoming.request.as_str() {
            "get_maps_path" => {
                let mut paths :Vec<PathBuf> = Vec::new();
                for map in &self.maps{
                    paths.push(map.path.clone());
                }
                let a_response = PathsResponse {
                    response_type: "get_maps_path".to_string(),
                    paths,
                };
                response = Ok(serde_json::to_string_pretty(&a_response).unwrap())
            },
            "get_map" => {
                let mut found = false;
                for mapfile in &mut self.maps{
                    if mapfile.path == incoming.content {
                        found = true;
                        mapfile.load();
                        let option_map = &mapfile.map;
                        match option_map {
                            Some(map) => {
                                response = map_to_yaml(&map);
                            }
                            None => {
                                let a_response = ErrorResponse {
                                    response_type: "error".to_string(),
                                    error: "Not able to load map! :-(".to_string(),
                                };
                                response = Ok(serde_json::to_string_pretty(&a_response).unwrap());
                            }
                        }
                    }
                }
                if ! found {
                    let a_response = ErrorResponse {
                        response_type: "error".to_string(),
                        error: "Not able to find the map! :-(".to_string(),
                    };
                    response = Ok(serde_json::to_string_pretty(&a_response).unwrap())
                }
            }
            _ => {
                println!("Unknown request {}", incoming.request);
                let a_response = UnknownRequestResponse{
                    response_type: "unknown_request".to_string(),
                    request: incoming.request
                };
                response = Ok(serde_json::to_string_pretty(&a_response).unwrap())
            },
        }
        response
    }
}