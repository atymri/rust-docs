use std::io::{self, Write};
use rand::prelude::*;
use rand::Rng;
use colored::Colorize;

fn main() {
    game();
}

fn game(){
    const WORDS: [&str; 220] = [
    "hello", "world", "game", "rust", "hell", "word", "guess", "input", "output", "color",
    "apple", "banana", "orange", "grape", "lemon", "peach", "cherry", "melon", "berry", "mango",
    "house", "home", "room", "door", "window", "table", "chair", "desk", "floor", "wall",
    "water", "fire", "earth", "wind", "storm", "cloud", "rain", "snow", "light", "dark",
    "sun", "moon", "star", "planet", "space", "comet", "orbit", "solar", "lunar", "cosmos",
    "code", "debug", "build", "compile", "binary", "string", "array", "vector", "slice", "tuple",
    "struct", "enum", "trait", "macro", "cargo", "crate", "borrow", "lifetime", "memory", "stack",
    "heap", "pointer", "unsafe", "async", "await", "thread", "mutex", "channel", "future", "spawn",
    "keyboard", "screen", "mouse", "button", "cursor", "pixel", "frame", "render", "shader", "texture",
    "music", "sound", "audio", "video", "movie", "photo", "image", "canvas", "draw", "paint",
    "quick", "slow", "fast", "speed", "boost", "power", "energy", "force", "motion", "gravity",
    "jump", "run", "walk", "climb", "crawl", "fly", "swim", "dodge", "slide", "fall",
    "enemy", "friend", "player", "boss", "level", "score", "point", "bonus", "quest", "mission",
    "magic", "spell", "sword", "shield", "armor", "arrow", "spear", "blade", "battle", "fight",
    "robot", "alien", "ghost", "zombie", "monster", "dragon", "wizard", "knight", "ninja", "pirate",
    "forest", "river", "ocean", "beach", "island", "mountain", "valley", "desert", "cave", "field",
    "bridge", "tower", "castle", "village", "city", "street", "road", "train", "plane", "ship",
    "green", "blue", "red", "yellow", "purple", "pink", "black", "white", "silver", "gold",
    "happy", "angry", "brave", "smart", "clever", "quiet", "loud", "crazy", "funny", "weird",
    "brain", "heart", "blood", "nerve", "muscle", "bone", "skin", "tooth", "finger", "shoulder",
    "clock", "timer", "alarm", "minute", "second", "hour", "today", "tomorrow", "yesterday", "future",
    "dream", "think", "learn", "teach", "write", "read", "speak", "listen", "watch", "create"];

    //let word = WORDS.choose(&mut rand::rng()).unwrap().to_string();
    let word = WORDS[rand::rng().random_range(0..WORDS.len())].to_string();
    let word_chars:Vec<char> = word.chars().collect();

    let mut guessed_positions:Vec<usize> = vec![];
    let mut guessed_chars:Vec<char> = vec![];

    let mut won = false;
    let mut failed_count = 0;

    print!("\x1B[2J\x1B[H");
    //tmp
    println!("{}", word.yellow());

    println!("{} chars", word.len());
    while !won{
        let mut guess = get_input(failed_count).trim().to_string();

        while guess.trim().len() != word.len(){
            println!("try again");
            guess = get_input(failed_count).trim().to_string();
        }

        //print!("\x1B[2J\x1B[H");

        for i in 0..word.len()
        {
            let word_char = word.as_bytes()[i] as char;
            let guess_char = guess.as_bytes()[i] as char;

            if word_char == guess_char && !guessed_positions.contains(&i){
                guessed_positions.push(i);
                guessed_chars.push(guess_char);
                print!("{}", guess_char.to_string().green());
            }else if word_chars.contains(&guess_char) && !guessed_chars.contains(&guess_char){
		        guessed_positions = vec![];
                guessed_chars = vec![];
                print!("{}", guess_char.to_string().yellow());
            }
            else{
                guessed_positions = vec![];
                print!("{}", guess_char.to_string());
            }
        }
        println!();


        if guess == word{
            won = true;
            println!("YOU WIN!\nWORD:{}", word.green());
        }else{
            failed_count += 1;
        }

        if failed_count == 5{
            println!("YOU LOSE!\nWORD: {}", word.red());
            break;
        }
    }
}

    

fn get_input(failed_count:i32) -> String{
    print!("({} of 5)-> ", failed_count);
    io::stdout().flush().unwrap();

    let mut input:String = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("invalid input");

    input
}
