use serde::{Serialize, Deserialize};

// Game structures
pub type PDaemon = State;
pub type RDaemon = State;

use std::fmt;

#[derive(Debug, Hash)]
pub enum State {
    Rock,
    Scissors,
    Paper,
    Display,
    Uninit,
}

#[derive(Debug, Hash, Deserialize, Serialize, Clone)]
pub struct Player {
    name: String,
    score: usize,
    rank: usize,
    stateid: usize,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name: name.to_string(),
            score: 0,
            rank: 0,
            stateid: 0,
        }
    }

    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn set_score(&mut self, val: usize) {
        self.score += val
    }

    pub fn reset(&mut self) {
        self.score = 0;
        self.rank = 0;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn update_state(&mut self, daemon: usize) {
        self.stateid = daemon
    }

    pub fn get_stateid(&self) -> usize {
        self.stateid
    }

}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Player: {}\nScore: {}\nRank: {}\nStateId: {:?}", 
               self.name, self.score, self.rank, self.stateid)
    }
}

#[derive(Debug, Hash)]
pub struct Leaderboard {
    top: String,
    score: usize,
    round: usize,
}

impl Leaderboard {
    pub fn init() -> Self {
        Leaderboard {
            top: String::new(),
            score: 0,
            round: 0,
        }
    }

    pub fn set_topper(&mut self, name: &str) {
        self.top = name.to_string();
    }

    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn update_score(&mut self, val: usize) {
        self.score = val
    }

    pub fn get_round(&self) -> usize {
        self.round
    }

    pub fn update_round(&mut self, inc: usize) {
        self.round += inc
    }

    pub fn renew(&mut self) {
        self.top = String::from("-");
        self.score = 0;
        self.round = 0;
    }
}


#[derive(Debug, Hash, Serialize, Eq, PartialEq, Deserialize, Clone)]
pub struct Gameboard {
    victor: Option<String>,
    r_score: usize,
    p_score: usize,
    rounds: usize,
}

impl Gameboard {

    pub fn update_victor(&mut self, name: Option<String>) {
        self.victor = name;
    }

    pub fn update(&mut self, r_val: usize, p_val: usize, inc: usize) {
        self.r_score = r_val;
        self.p_score = p_val;
        self.rounds = inc;
    }

    pub fn get_r_score(&self) -> usize {
        self.r_score
    }

    pub fn get_p_score(&self) -> usize {
        self.p_score
    }

    pub fn get_rounds(&self) -> usize {
        self.rounds
    }

    pub fn init() -> Self {
        Gameboard {
            victor: None,
            p_score: 0,
            r_score: 0,
            rounds: 0,
        }
    }

    pub fn display_result(&self) {
        println!("{}", &self);
    }
}

impl fmt::Display for Gameboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let winner: Option<String> = self.victor.clone();
        let win = winner.unwrap();
        write!(f, "=======================================================
Player: {}\nRobo: {}\nTotal rounds: {}\nWinner: {:?}
=======================================================", 
self.p_score, self.r_score, self.rounds, win)
    }
}

#[derive(Debug, Hash, Serialize, Eq, PartialEq, Deserialize)]
pub struct GameSum {
    state: Gameboard,
    player: Vec<Player>,
}

impl GameSum {
    pub fn new(gamestate: Gameboard, player: Vec<Player>) -> Self {
        GameSum { state: gamestate, player: player }
    }

    pub fn get_players(&self) -> &Vec<Player> {
        &self.player
    }

    pub fn get_mut_players(&mut self) -> &mut Vec<Player> {
        &mut self.player
    }

}

impl fmt::Display for GameSum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, " Game: {}\t Player: {:?}", self.state, self.player)
    }
}
