use std::{io, vec};
use colored::Colorize;

fn main() {
    print!("\x1B[2J\x1B[1;1H");
    let mut game_over: bool = false;
    let word = get_word();
    let chars = make_chars(&word);
    let mut slots = create_slots(word.len());
    pretty_print(&slots);
    let mut state_idx = 0;
    let mut state = "zero".to_string();
    state_machine(state.clone().into(), state_idx);
    while !game_over {
        print!("\x1B[2J\x1B[1;1H");
        let msg = "-> Word has been chosen by player 2".yellow();
        println!("{}", msg);
        state_machine(state.clone().into(), state_idx);
        pretty_print(&slots);
        let guess = get_guess();
        (slots, state) = update_slots(&chars, guess, &mut slots);
        if state == "w" {
            let str = "Cool!".to_string().blue();
            println!("{}", str);
        } else {
            state_idx += 1;
            let str = "Wrong!".to_string().red();
            println!("{}", str);
        }
        state_machine(state.clone().into(), state_idx);
        game_over = check_game_over(&slots);
        if state_idx >= 5 {
            game_over = true;
            print!("\x1B[2J\x1B[1;1H");
            state_machine(state.clone().into(), state_idx);
        }
    }
    //pretty_print(&slots);
}

fn get_word() -> String {

    loop {
        let title = r#" ___  ___  ________  ________   ________  _______   ________ ________  ___       
|\  \|\  \|\   __  \|\   ___  \|\   ____\|\  ___ \ |\   __  \\_____  \|\  \      
\ \  \\\  \ \  \|\  \ \  \\ \  \ \  \___|\ \   __/|\ \  \|\  \|___/  /\ \  \     
 \ \   __  \ \   __  \ \  \\ \  \ \  \  __\ \  \_|/_\ \   _  _\  /  / /\ \  \    
  \ \  \ \  \ \  \ \  \ \  \\ \  \ \  \|\  \ \  \_|\ \ \  \\  \|/  /_/__\ \__\   
   \ \__\ \__\ \__\ \__\ \__\\ \__\ \_______\ \_______\ \__\\ _\\________\|__|   
    \|__|\|__|\|__|\|__|\|__| \|__|\|_______|\|_______|\|__|\|__\|_______|   ___ 
                                                                            |\__\
                                                                            \|__|"#.green();
        let prompt = "-> plz enter a word:".red();
        println!("{}\n{}", title, prompt);

        let mut input = String::new();

        let err = "Failed to read line!".red();
        io::stdin()
            .read_line(&mut input)
            .expect(&err);

        let input: String = match input.trim().parse() {
            Ok(word) => word,
            Err(_) => continue,
        };
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
    println!("{}", slots_str.yellow())
}

fn get_guess() -> char {
    loop {
        let prompt = "-> Guess a char:".green();
        println!("{}", prompt);

        let mut input = String::new();

        let err = "Failed to read line!".red();
        io::stdin()
            .read_line(&mut input)
            .expect(&err);

        let input: char = match input.trim().parse() {
            Ok(word) => word,
            Err(_) => {
                let str = "Only one character bitch!".to_string().red();
                println!("{}", str);
                continue
            }

        };
        let response = "-> Your letter:".green();
        println!("{} {}", response, input.to_string().red());

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

fn update_slots(chars: &Vec<char>, guess: char, 
    slots: &mut Vec<String>) -> (Vec<String>, String) {
    let mut state = "l".to_string();
    chars
        .into_iter()
        .enumerate()
        .for_each(|(idx, item)| {
            if *item == guess {
                state = "w".to_string();
                slots[idx] = guess.to_string();
            } 
        });
    return (slots.to_vec(), state)
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
        print!("\x1B[2J\x1B[1;1H");
        let won_str = "You won bitch!".green();
        let other_str = "The word was".yellow();
        println!("{}", won_str);
        println!("{} '{}'!", other_str, slots.join("").red());
        let state_idx = 6;
        let state = "w";
        state_machine(state.into(), state_idx);
        return true;        
    }
    false
}

fn state_machine(state: String, state_idx: usize) {
    let state_vec = vec![
        r#"
    _________
    |/      |
    |      
    |      
    |       
    |      
    |
 ___|___
    "#,
        r#"
    _________
    |/      |
    |      (_)
    |      
    |       
    |      
    |
 ___|___
    "#,
        r#"
    _________
    |/      |
    |      (_)
    |       |
    |       |
    |      
    |
 ___|___
    "#,
        r#"
    _________
    |/      |
    |      (_)
    |       |
    |      /|
    |      
    |
 ___|___
    "#,
        r#"
    _________
    |/      |
    |      (_)
    |       | 
    |      /|\
    |      /
    |
 ___|___
    "#,
        r#"
    _________
    |/      |
    |      (_)    HAHA! 
    |       |     What a Loser!
    |      /|\    You Died Horribly! and we cheered!
    |      / \
    |
 ___|___\o/\o/\o/\o/
    "#,
        r#"
    _________
    |/      
    |            Boo! 
    |       O    I guess you live for another day!
    |      \|/
    |       |
    |      / \
 ___|___ o  o  o  o   
    "#,
    ];
    if state == "l" {
        let state_str = state_vec[state_idx].red();
        println!("{}", state_str);
        return
    } else if state == "w" {
        let state_str = state_vec[state_idx].green();
        println!("{}", state_str);
        return
    } else {
        let state_str = state_vec[state_idx].yellow();
        println!("{}", state_str);
        return
    }
}
