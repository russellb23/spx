extern crate serde;
#[macro_use]
extern crate serde_derive;

extern crate rand;

pub mod base;
pub mod base_impl;
pub mod markov;

use std::io::stdin;
use std::char;
use std::f32::INFINITY;

use base::Player;
use base::Gameboard;
use base::GameSum;
use base::State;

use std::collections::HashMap;
//use std::result::Result;

use rand::Rng;

pub fn game(pname: String, num_rounds: usize) {
    let mut done = false;

    let mut ghistory: HashMap<usize, GameSum> = HashMap::new();

    let mut player = Player::new(pname);
    let mut robo = Player::new(String::from("Robot"));

    let mut g: Gameboard = Gameboard::init();

    let mut nround = 0;
    while !done {
        nround += 1;
        let (p_daem, r_daem) = get_daemons(&mut player, &mut robo);

        if nround >= 10 && nround < num_rounds {
            let state_prob = markovp(&ghistory, nround);
            let rp = r_daem.get_stateid();
            let mut cur_state: Vec<f32> = Vec::with_capacity(3);

            for i in 0..3 {
                let mut p = char::from_digit(rp as u32, 10).unwrap().to_string();
                let c = char::from_digit(i+1 as u32, 10).unwrap().to_string();
                p.push_str(c.as_str());
                match state_prob.get(&p) {
                    Some(val) => cur_state.push(*val),
                    None => cur_state.push(INFINITY),
                }
            }
            println!("r: {:.1}       \t p: {:.1}\t\t s: {:.1}\t\t [{}:{}]", 
&cur_state[0], &cur_state[1], &cur_state[2], g.get_p_score(), g.get_r_score());
        }
        
        let r = r_daem.get_stateid();
        let p = p_daem.get_stateid();

        g.update(robo.get_score(), player.get_score(), nround);

        if r == 4 || p == 4 { 
            g.display_result();
        } else {
            if r > p {
                if r == 2 && p == 1 {
                    player.set_score(1);
                } else if r == 3 && p == 1 {
                    robo.set_score(1);
                } else if r == 3 && p == 2 {
                    player.set_score(1);
                }
            } else if r < p {
                if r == 2 && p == 3 {
                    robo.set_score(1);
                } else if r == 1 && p == 3 {
                    player.set_score(1);
                } else if r == 1 && p == 2 {
                    robo.set_score(1);
                }
            } else if r == p {
                player.set_score(0);
                robo.set_score(0);
            }
        }
        // Set victor 
        if g.get_p_score() > g.get_r_score() {
            g.update_victor(Some(player.get_name().to_string()));
        } else if g.get_r_score() > g.get_p_score() {
            g.update_victor(Some(robo.get_name().to_string()));
        } else {
            g.update_victor(Some(String::from("TIE")));
        }

        ghistory.insert(nround, GameSum::new(g.clone(), vec![robo.clone(), 
                                                             player.clone()]));

        if nround >= num_rounds {
            println!("{}", &g);
            done = true;
        }
    }

}


fn get_daemons<'a>(p: &'a mut Player, r: &'a mut Player) -> 
                                            (&'a mut Player, &'a mut Player) { 
    let robo = rand::thread_rng().gen_range(1,4);
    let play = player_daemon();

    match robo {
        1 => { r.update_state(1) },
        2 => { r.update_state(2) },
        3 => { r.update_state(3) },
        _ => { r.update_state(4) },
    };
    match play {
        State::Rock => { p.update_state(1) },
        State::Paper => { p.update_state(2) },
        State::Scissors => { p.update_state(3) },
        _ => { p.update_state(4) },
    };
    (r, p)
}

fn player_daemon() -> State {
    let mut daemon = String::new();
    let mut noval = false;

    while !noval {
        println!("r: Rock    \t p: Paper\t s: Scissors\t x|q: Exit");
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
        _ => { println!("\tUnrecognized choice"); player_daemon() },
    }
}

fn markovp(game_history: &HashMap<usize, GameSum>, nthround: usize) -> HashMap<String, f32> {
    
    let analwin: usize = 9;
    let trange: (usize, usize) = (nthround-analwin,nthround);
    let mut dat: Vec<usize> = Vec::with_capacity(analwin);
    
    for key in trange.0..trange.1 {
        let k = game_history.get(&key);
        let v = &k.unwrap().get_players()[0]; //1st position is robot
        dat.push(v.get_stateid());
    }

    let pmat = markov::tranprob(&dat);
    pmat 
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
