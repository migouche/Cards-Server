use std::{borrow::Borrow, sync::Arc};
use tokio::sync::{Mutex, MutexGuard};
use std::net::SocketAddr;

use futures::{FutureExt, SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{accept_async, tungstenite::Message};

use crate::game::GameState;

fn trim_newline(s: &mut String) {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
}

fn disconnect_player(ip: &SocketAddr, game_state: &mut MutexGuard<GameState>)
{
    println!("{}@{} disconnected!", game_state.get_player(ip).unwrap().get_name() ,ip);
    game_state.remove_player(ip);

}

pub async fn handle_connection(stream: TcpStream, game_state: Arc<Mutex<GameState>>)
{
    let ws_stream = accept_async(stream).await.expect("couldn't handshake websocket");

    println!("Created websocket connection");

    let peer_addr = ws_stream.get_ref().peer_addr().expect("couldn't get peer address");

    println!("{} connected!",peer_addr);


    let (mut write, mut read) = ws_stream.split();

    let mut name = read.next().await.expect("Couldn't get name").unwrap().to_string();
    trim_newline(&mut name);
    println!("Your name is: {}", name);
    write.send(format!("Welcome, {}!", name).into()).await.expect("Couldn't send welcome message");
    game_state.lock().await.add_player(&name, peer_addr);


    while let Some(message) = read.next().await {
        let mut state = game_state.lock().await;

        if let Err(e) = message
        {
            disconnect_player(&peer_addr, &mut state);
            break;
            
        }
        let message = message.unwrap();


        let mut text = message.to_string();
        trim_newline(&mut text);


        if text == "exit"
        {
            disconnect_player(&peer_addr, &mut state);
            break;
        }

        println!("message from {}@{}: {}",state.get_player(&peer_addr).unwrap().get_name() ,peer_addr,  text);
        // do stuff with state
        write.send(message).await.expect("couldnt send message");
    }
    println!("closing connection with {}!",peer_addr);
}