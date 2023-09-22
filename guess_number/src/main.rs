// Guess the number game
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // generate a random number between 0 and 100
    // let secret_number = rand::random::<u32>() % 100;
    let secret_number = rand::thread_rng().gen_range(0..100);

    println!("Please input your guess:");
    let mut input = String::new();
    let mut cnt = 0;

    loop {
        // clean the input
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // parse the input to u32
        let guess: u32 = match input.trim().parse() {
            Ok(num) => {
                cnt += 1;
                num
            }
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        // if guess == secret_number {
        //     succesed = true;
        //     println!("You win!");
        // } else if guess < secret_number {
        //     println!("Too small!");
        // } else {
        //     println!("Too big!");
        // }

        // or use cmp() to compare two strings
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        if cnt == 5 {
            println!("You lose! The secret number is {}", secret_number);
            break;
        }
    }
}
