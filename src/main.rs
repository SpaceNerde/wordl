use std::io::{self, BufRead, Stdin};

const WORD_LIST: &str = include_str!("../assets/twl06.txt");

fn clean_input(input: &str) -> String {
    input.trim()
        .to_lowercase()
        .chars().filter(|c| c.is_ascii_alphabetic())
        .collect()
} 

fn process_input(stdin: Stdin) -> Option<String> {
    let mut handle = stdin.lock();
    for line in handle.lines() {
        return Some(line.unwrap());
    }

    Some("1".to_string()) 
}

fn select_word(length: usize) -> String {
    let list: Vec<String> = WORD_LIST
        .split("\n")
        .skip(2)
        .map(clean_input)
        .filter(|l| l.len() == length)
        .collect();
    list[0].clone()
}

enum State {
    Setup,
    Playing,
    Loss,
    Win,
}

fn main() {
    let mut max_word_length: usize = 5;
    
    let mut current_state = State::Setup;

    println!("--SELECT YOUR WORD LENGTH--");

    loop {
        let mut buffer = String::new();
        let stdin = io::stdin();
        
        
        match current_state {
            State::Setup => {
                max_word_length = process_input(stdin).unwrap().parse().unwrap();
                println!("{}", max_word_length);
            },
            State::Playing => {
                            },
            State::Loss => {

            },
            State::Win => {

            },
        }
    }
}
