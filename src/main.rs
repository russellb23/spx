extern crate spx;

use spx::*;
use std::io::stdin;

fn prompt() -> i8 {
    // Predefined number of rounds
    println!("\n ======================================================== \n");
    println!("No rounds:\n1) 10 \n2) 20 \n3) 30 \n4) 50 \n5) Custom \n6) Quit");
    let mut _nrounds = String::new();
    stdin().read_line(&mut _nrounds).expect("Failed to get the choice of rounds");
    let nrounds: i8 = match _nrounds.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Unknown error"),
    };
    nrounds
}

fn custom_rounds() -> usize {
    println!("Custom rounds: ");
    let mut cus_round = String::new();
    stdin().read_line(&mut cus_round).expect("Failed to read value");

    match cus_round.trim().parse::<usize>() {
        Ok(val) => val,
        Err(_) => panic!("No parseable number"),
    }
}

fn main() {
    println!("          Welcome to Stone <==> Paper <==> Scissors
                            /|\\
                          _( . )_
                        _/ (!+!) \\_
                      _(-^-^-^-^-^-)_
                      <^(O)! = !(O)^>
                       \\___________/");


    println!("\n\nPlayer Name: ");
    let mut pname = String::new();
    stdin().read_line(&mut pname).expect("Failed to read name");

    let name: String = match pname.trim().parse() {
        Ok(nam) => nam,
        Err(_) => panic!("Not able to parse name"),
    };

    let nr = prompt();

    match nr {
        1 => { game(name, 10) },
        2 => { game(name, 20) },
        3 => { game(name, 30) },
        4 => { game(name, 50) },
        5 => { let nn = custom_rounds(); game(name, nn) },
        6 => { ::std::process::exit(1) },
        _ => { ::std::process::exit(1) },
    }

}
