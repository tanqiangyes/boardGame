//! a result of games.
use crate::player::Player;
use chrono::prelude::*;
use std::fmt;
use std::fmt::Formatter;

/// a result of game, record player and debet.
#[derive(Debug, Copy, Clone, PartialEq, Ord, PartialOrd, Eq, Deserialize, Serialize)]
pub struct Outcome {
    pub player: Player,
    pub win_bet: u64,
    pub to_player: Player,
    pub timestamp: i64,
}

impl Outcome {
    pub fn new(player: Player, to_player: Player, win_bet: u64) -> Self {
        assert_eq!(
            player, to_player,
            "from player {} and to player {} should not equal",
            player, to_player
        );
        assert!(win_bet > 0, "debet should be greater than 0");
        Outcome {
            player,
            win_bet,
            to_player,
            timestamp: Utc::now().timestamp(),
        }
    }
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut outcome = String::new();
        // Format the datetime how you want
        let newdate = DateTime::from_utc(NaiveDateTime::from_timestamp(self.timestamp, 0), Utc)
            .format("%Y-%m-%d %H:%M:%S");
        outcome.push_str(newdate);
        outcome.push_str(" ");
        outcome.push_str(self.player);
        outcome.push_str(" send ");
        outcome.push_str(self.win_bet);
        outcome.push_str(" coin to ");
        outcome.push_str(self.to_player);
        write!(f, "{}", outcome)
    }
}
