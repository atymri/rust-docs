use colored::Colorize;
use std::io::{self, Write};

// guessing game! i generate a number, you have to guess.
fn main() {
    print!("1.guess digit by digit\n2.guess the whole number\n->");
    io::stdout()
        .flush().expect("failed to flush");
        
        let number:i8 = get_input().trim().parse().expect("invalid number");


    match number {
        1 => guessing_digit(),
        2 => guessing_number(),
        _ => println!("invalid input")
    }
}

fn get_input() -> String {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("invalid input");

    input
}

fn guessing_digit() {
    let generated: u16 = rand::random::<u16>();
    let generated_str = generated.to_string();
    let mut digits: Vec<char> = vec![];
    let mut guessed_positions: Vec<usize> = vec![];

    let mut won = false;

    for i in 0..generated_str.len() {
        digits.push((generated_str.as_bytes()[i]) as char);
    }

    println!("{}", generated);
    while !won {
        println!();
        let guessed: u16 = get_input().trim().parse().expect("invalid number");
        let guessed_str = guessed.to_string();
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().expect("failed to flush");

        if guessed_str.len() != generated_str.len(){
            println!("try again");
        } else {
            for i in 0..generated_str.len() {
                let digit = guessed_str.as_bytes()[i] as char;
                if digits[i] == digit {
                    if !guessed_positions.contains(&i){
                        guessed_positions.push(i);
                    }
                    print!("{}", digit.to_string().green());
                } else if generated_str.contains(digit){
                    print!("{}", digit.to_string().yellow());
                }else{
                    print!("{}", digit)
                }
            }
            if guessed_positions.len() == generated_str.len() || guessed == generated {
                won = true;
                println!("YOU WiN!");
            }
        }
    }
}

fn guessing_number() {
    let generated: i8 = rand::random::<i8>();
    let mut found: bool = false;

    println!("{}", generated);

    while !found {
        print!("guess a number: ");
        io::stdout().flush().expect("failed to flush");

        let number: i8 = get_input().trim().parse().expect("not a number");
        if number > generated {
            println!("BIG!");
        } else if number < generated {
            println!("SMALL!");
        } else {
            println!("YOU WON!");
            found = true;
        }
    }
}
