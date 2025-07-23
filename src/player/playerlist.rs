use crate::ai::AIStrategy;
use crate::player::base_player::*;

pub struct PlayerList<'a> {
    player_1: Player<'a>,
    player_2: Player<'a>,
}

pub struct PlayerListIterator<'a> {
    players: &'a PlayerList<'a>,
    index: usize,
}

// I am doing this primarily for fun and learning, and so I can iterate over players
impl<'a> Iterator for PlayerListIterator<'a> {
    type Item = &'a Player<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.index {
            0 => {
                self.index += 1;
                Some(&self.players.player_1)
            }
            1 => {
                self.index += 1;
                Some(&self.players.player_2)
            }
            _ => None,
        }
    }
}

impl<'a> Default for PlayerList<'a> {
    fn default() -> Self {
        PlayerList {
            player_1: Player::new("x", PlayerPiece::new(1), PlayerType::Local),
            player_2: Player::new(
                "y",
                PlayerPiece::new(2),
                PlayerType::AI(AIStrategy::Minimax),
            ),
        }
    }
}

impl<'a> PlayerList<'a> {
    /// This is currently wrapped in an Option, to reflect the potential for multiplayer in the future
    pub fn get_ai_player(&self) -> Option<&Player> {
        let ai_players = self.iter().filter(|p| p.is_ai()).collect::<Vec<&Player>>();

        if ai_players.is_empty() {
            None
        } else {
            Some(
                ai_players
                    .first()
                    .expect("Unrecoverable error. AI Player exists but can't be found."),
            )
        }
    }

    pub fn get_local_human_player(&self) -> &Player {
        let local_player = self
            .iter()
            .filter(|p| p.is_local())
            .collect::<Vec<&Player>>();

        if local_player.is_empty() {
            panic!(
                "Well. That's awkward. No local human player found, which should not be possible."
            )
        } else {
            local_player.first().unwrap()
        }
    }

    pub fn get_ai_player_piece(&self) -> Option<PlayerPiece> {
        self.get_ai_player().map(|p| p.player_piece)
    }

    /// Returns the local human player's piece. This is assumed to always exist.
    pub fn get_local_human_player_piece(&self) -> PlayerPiece {
        self.get_local_human_player().player_piece
    }

    pub fn iter(&self) -> PlayerListIterator {
        PlayerListIterator {
            players: self,
            index: 0,
        }
    }
}
