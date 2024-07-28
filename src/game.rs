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
            name: "a".to_string(),
            hand: Vec::new()
        }
    }
}


struct GameData
{
    rounds: u8,
    phases: u8,
}

pub struct GameState
{
    pub players: Vec<Player>,
}

impl GameState {
    pub fn new() -> Self
    {
        GameState
        {
            players: Vec::new()
        }
    }
}
