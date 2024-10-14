use std::{io, vec};
use colored::Colorize;

fn main() {
    let mut game_over: bool = false;
    let word = get_word();
    let chars = make_chars(&word);
    let mut slots = create_slots(word.len());
    pretty_print(&slots);
    while !game_over {
        let guess = get_guess();
        let slots = update_slots(&chars, guess, &mut slots);
        pretty_print(&slots);
        game_over = check_game_over(&slots);
    }
    //pretty_print(&slots);
}

fn get_word() -> String {

    loop {

        println!("-> Multi-player Hangman ONLINE! \n-> plz enter a word:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let input: String = match input.trim().parse() {
            Ok(word) => word,
            Err(_) => continue,
        };

        println!("-> You choose these word: {}!", input);

        return input;
    }

}

fn create_slots(word_size: usize) -> Vec<String> {
    let mut slot_vec = vec![];
    for _ in 0..word_size {
        slot_vec.push("_".to_string());
    }
    return  slot_vec;
}

fn pretty_print(slots: &Vec<String>) {
    let slots_str = slots.join(" . ");
    println!("{}", slots_str.green())
}

fn get_guess() -> char {

    loop {

        println!("-> Guess:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let input: char = match input.trim().parse() {
            Ok(word) => word,
            Err(_) => {
                let str = "Only one character bitch!".to_string().red();
                println!("{}", str);
                continue
            }

        };

        println!("-> Your letter: {}!", input);

        return input;
    }
}

fn make_chars(word: &String) -> Vec<char> {
    let mut word_vec = vec![];
    let _ = word
        .chars().for_each(|c| word_vec.push(c));
    println!("{:?}", word_vec);
    return word_vec
}

fn update_slots(chars: &Vec<char>, guess: char, slots: &mut Vec<String>) -> Vec<String> {
    chars
        .into_iter()
        .enumerate()
        .for_each(|(idx, item)| {
            if *item == guess {
                println!("{}", idx);
                slots[idx] = guess.to_string();
            }
        });
    return slots.to_vec()
}

fn check_game_over(slots: &Vec<String>) -> bool {
    let mut blanks: usize = 0;
    slots
        .into_iter()
        .for_each(|item| {
            if *item == "_"{
                blanks += 1;
            }
        });
    if blanks == 0 {
        return true;        
    }
    false
}
