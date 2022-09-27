use crate::outcome::Outcome;
use crate::player::Player;
use anyhow::Result;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Ord, PartialOrd, Eq, Deserialize, Serialize)]
pub struct Room {
    room_id: uuid::Uuid,
    game_id: u64,
    players: Vec<Player>,
    result: Vec<Outcome>,
}

impl Room {
    pub fn new(room_id: uuid::Uuid, game_id: u64) -> Self {
        Room {
            room_id: room_id,
            game_id: game_id,
            players: Vec::new(),
            result: Vec::new(),
        }
    }

    pub fn new_with_players(room_id: uuid::Uuid, game_id: u64, players: Player) -> Self {
        Room {
            room_id: room_id,
            game_id: game_id,
            players: vec![players],
            result: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player)
    }

    pub fn add_players(&mut self, players: Vec<Player>) {
        self.players.append(players)
    }

    pub fn get_result(&self) -> Vec<Outcome> {
        Ok(self.result)
    }
}
