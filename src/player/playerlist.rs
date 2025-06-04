use crate::player::playerbase::*;

pub struct PlayerList<'a> {
    player_1: Player<'a>,
    player_2: Player<'a>,
}

pub struct PlayerListIterator<'a> {
    players: &'a PlayerList<'a>,
    index: usize,
}

pub struct PlayersInfo {
    pub ai_piece: u8,
    pub player_piece: u8,
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
            _ => {
                self.index = 0;
                None
            }
        }
    }
}

impl<'a> Default for PlayerList<'a> {
    fn default() -> Self {
        PlayerList {
            player_1: Player::new("x", 32, PlayerType::Local),
            player_2: Player::new("y", 64, PlayerType::AI(AIStrategy::Minimax)),
        }
    }
}

impl<'a> PlayerList<'a> {
    pub fn get_ai_player_piece(&self) -> u8 {
        if self.player_1.player_type() == PlayerType::Local
            || self.player_1.player_type() == PlayerType::Remote
        {
            self.player_2.encoded
        } else {
            self.player_1.encoded
        }
    }

    pub fn get_players_piece_info(&self) -> PlayersInfo {
        PlayersInfo {
            ai_piece: self.get_ai_player_piece(),
            player_piece: self.get_human_player_piece(),
        }
    }

    // Not DRY
    pub fn get_human_player_piece(&self) -> u8 {
        if self.player_1.player_type() == PlayerType::Local
            || self.player_1.player_type() == PlayerType::Remote
        {
            self.player_1.encoded
        } else {
            self.player_2.encoded
        }
    }

    pub fn iter(&self) -> PlayerListIterator {
        PlayerListIterator {
            players: self,
            index: 0,
        }
    }
}
