use std::{borrow::Borrow, sync::Arc};
use tokio::sync::Mutex;

use futures::{FutureExt, SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_tungstenite::accept_async;

use crate::game::GameState;



pub async fn handle_connection(stream: TcpStream, game_state: Arc<Mutex<GameState>>)
{
    let ws_stream = accept_async(stream).await.expect("couldn't handshake websocket");

    println!("Created websocket connection");

    let (mut write, mut read) = ws_stream.split();
    while let Some(message) = read.next().await {
            let message = message.expect("Couldn't read message");
            let mut state = game_state.lock().await;
            println!("{:?}", state.players);
            // do stuff with state
            write.send(message).await.expect("couldnt send message");
    }
}