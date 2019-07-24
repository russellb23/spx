extern crate rand;

pub mod base;
pub mod base_impl;

use std::io::stdin;
use base::Player;
use base::GameState;
use base::Gameboard;
use base::State;
//use base::RDaemon;
//use base::PDaemon;

use std::collections::HashMap;
//use std::result::Result;
//
use rand::Rng;


pub fn game(pname: String, num_rounds: usize) {
    let mut done = false;
    let mut player = Player::new(pname);
    let mut airobo = Player::new(String::from("Robot"));
    let g: Gameboard = Gameboard::init(None, None);
    let mut nround = 0;
    while !done {
        nround += 1;
        let (r_daem, p_daem) = get_daemons(&mut player, &mut airobo);
        let r = r_daem.get_stateid();
        let p = p_daem.get_stateid();

        if r == 4 || p == 4 { 
            g.display_result();
        } else {
            if r > p {
                if r == 2 && p == 1 {
                    player.add_score(1);
                } else if r == 3 && p == 1 {
                    airobo.add_score(1);
                } else if r == 3 && p == 2 {
                    player.add_score(1);
                }
            } else if r < p {
                if r == 2 && p == 3 {
                    airobo.add_score(1);
                } else if r == 1 && p == 3 {
                    player.add_score(1);
                } else if r == 1 && p == 2 {
                    player.add_score(1);
                }
            } else if r == p {
                p;
            }
        }

        if nround >= num_rounds {
            println!("{}", &g);
            done = true;
        }
    }
}


pub fn get_daemons<'a>(r: &'a mut Player, p: &'a mut Player) -> 
                                            (&'a mut Player, &'a mut Player) { 
    let robo = rand::thread_rng().gen_range(1,4);
    let play = player_daemon();

//    let mut robo_state: State;
//    let mut player_daemon: State;

    match robo {
        1 => { r.update_state(1) },
        2 => { r.update_state(2)},
        3 => { r.update_state(3)},
        _ => { r.update_state(4)},
    };
    match play {
        State::Rock => { p.update_state(1) },
        State::Paper => { p.update_state(2)},
        State::Scissors => { p.update_state(3)},
        _ => { p.update_state(4)},
    };
    (r, p)
}

pub fn player_daemon() -> State {
    let mut daemon = String::new();
    let mut noval = false;

    while !noval {
        println!("r: Rock\t p: Paper\t s: Scissors\t x|q: Exit");
        stdin().read_line(&mut daemon).expect("Failed to read");
    
        if daemon.trim().len() >= 1 {
            noval = true
        }
    }

    let state = daemon.trim().as_bytes()[0];

    match state {
        114 => return State::Rock,
        112 => return State::Paper,
        115 => return State::Scissors,
        120 => panic!("Game ends"),
        113 => return State::Display,
        _ => player_daemon(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}