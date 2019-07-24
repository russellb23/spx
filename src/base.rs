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

#[derive(Debug, Hash)]
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

    pub fn add_score(&mut self, val: usize) {
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
        self.score += val
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


#[derive(Debug, Hash)]
pub struct Gameboard {
    airobo: Option<State>,
    player: Option<State>,
    victor: Option<String>,
    r_score: usize,
    p_score: usize,
    rounds: usize,
}

impl Gameboard {
    pub fn update_states(&mut self, p: Option<State>, r: Option<State>) {
        self.airobo = r;
        self.player = p;
    }

    pub fn update_victor(&mut self, name: Option<String>) {
        self.victor = name;
    }

    pub fn update(&mut self, r_val: usize, p_val: usize, inc: usize) {
        self.r_score = r_val;
        self.p_score = p_val;
        self.rounds += inc;
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

    pub fn init(r: Option<State>, p: Option<State>) -> Self {
        Gameboard {
            airobo: r,
            player: p,
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
        write!(f, "Player: {}\nRobo: {}\nRound: {}", self.p_score, 
               self.r_score, self.rounds)
    }
}


#[derive(Debug, Hash)]
pub struct GameState {
    state: Gameboard,
}

impl GameState {
    pub fn new(gamestate: Gameboard) -> Self {
        GameState { state: gamestate, }
    }
}
