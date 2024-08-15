use std::{collections::{hash_map::Entry, HashMap}, net::SocketAddr};

#[derive(Debug)]
struct Card
{
    id: u8
}

#[derive(Debug)]
pub struct Player
{
    name: String,
    hand: Vec<Card>
}

impl Player {
    pub fn new(name: &str) -> Self
    {
        Player 
        {
            name: name.to_string(),
            hand: Vec::new()
        }
    }

    pub fn get_name(&self) -> &str
    {
        &self.name
    }
}




struct GameData
{
    rounds: u8,
    phases: u8,
}

pub struct GameState
{
    pub players: HashMap<SocketAddr, Player>
}

impl GameState {
    pub fn new() -> Self
    {
        GameState
        {
            players: HashMap::new()
        }
    }

    pub fn get_player(&self, ip: &SocketAddr) -> Option<&Player> {
        self.players.get(ip)
    }

    pub fn get_player_mut(&mut self, ip: &SocketAddr) -> Option<&mut Player> {
        self.players.get_mut(ip)
    }

    pub fn remove_player(&mut self, ip: &SocketAddr) -> Option<Player> {
        self.players.remove(ip)
    }

    pub fn add_player(&mut self, name: &str, ip: SocketAddr) -> Option<&mut Player>
    {
        match self.players.entry(ip)
        {
            Entry::Vacant(v) => Some(v.insert(Player::new(&name))),
            Entry::Occupied(_) => None
        }
    }
}
