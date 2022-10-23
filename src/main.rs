#[macro_use]
extern crate serde_derive;
extern crate indicatif;

use std::io;
use std::io::Write;
use std::process;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    get_input("input a miner address ", &mut miner_addr);
    get_input("Difficulty ", &mut difficulty);

    let diff = difficulty.trim().parse::<u32>().expect("we need");
    println!("generating genesis block!");

    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("(1) New Transaction");
        println!("(2) Mine block");
        println!("(3) Change Difficulty");
        println!("(4) Change Reward");
        println!("(0) Exit");
        print!("Enter your choice: ");
        io::stdout().flush();
        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("Exiting");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                get_input("enter sender address: ", &mut sender);
                get_input("enter receiver address: ", &mut receiver);
                get_input("enter amount: ", &mut amount);

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );
                match res {
                    true => println!("Transaction added"),
                    false => println!("Transaction failed"),
                }
            }
            2 => {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generated failed"),
                }
            }
            3 => {
                let mut new_diff = String::new();
                get_input("enter new difficulty: ", &mut new_diff);
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Updated difficulty"),
                    false => println!("Failed update difficulty"),
                }
            }
            4 => {
                let res = chain.update_reward();
                match res {
                    true => println!("Updated reward"),
                    false => println!("Failed update reward"),
                }
            }
            _ => println!("Invalid option please retry"),
        }
    }
}

fn get_input(ask_msg: &str, s: &mut String) {
    print!("{}", ask_msg);
    io::stdout().flush();
    io::stdin().read_line(s);
}
