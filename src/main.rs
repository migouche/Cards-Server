mod server;
mod game;

use std::sync::Arc;
use tokio::sync::Mutex;

use server::handle_connection;
use tokio::net::{TcpListener, TcpStream};
use clap::Parser;
use game::GameState;


#[derive(Parser)]
struct Args
{
    #[arg(short, long, default_value="7777")]
    port: u16
}



#[tokio::main]
async fn main() {
    let args = Args::parse();
    let addr = format!("127.0.0.1:{}", args.port);

    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");
    let game_state = Arc::new(Mutex::new(GameState::new()));
    println!("Listening on {}", addr);
    while let Ok((stream, _)) = listener.accept().await {
        let state = game_state.clone();
        tokio::spawn(handle_connection(stream, state));
        
    }

}
