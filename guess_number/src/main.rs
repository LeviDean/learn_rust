// Guess the number game
use std::io;

fn main() {
    // generate a random number between 0 and 100
    let secret_number = rand::random::<u32>() % 100;

    println!("Please input your guess:");
    let mut input = String::new();
    let mut succesed = false;
    let mut cnt = 0;

    while !succesed && cnt < 5 {

        // clean the input
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

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

        if guess == secret_number {
            succesed = true;
            println!("You win!");
        } else {
            println!("Not right!");
        }
    }
    
    if cnt == 5 {
        println!("You lose! The secret number is {}", secret_number);
    }
}
