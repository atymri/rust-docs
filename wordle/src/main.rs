use std::io::{self, Write};
use rand::seq::IndexedRandom;
use colored::Colorize;

fn main() {
    game();
}

fn game(){
    const WORDS: [&str; 10] = ["hello", "world", "game", "rust", "hell", "word", "guess", "input", "output", "color"];

    let word = WORDS.choose(&mut rand::rng()).unwrap().to_string();
    let word_chars:Vec<char> = word.chars().collect();

    let mut guessed_positions:Vec<usize> = vec![];
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

        for i in 0..word.len(){
            let word_char = word.as_bytes()[i] as char;
            let guess_char = guess.as_bytes()[i] as char;

            if word_char == guess_char && !guessed_positions.contains(&i){
                guessed_positions.push(i);
                print!("{}", guess_char.to_string().green());
            }else if word_chars.contains(&guess_char){
		        guessed_positions = vec![];
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
