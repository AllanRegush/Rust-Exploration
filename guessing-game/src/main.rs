use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is {}", secret_number);

    let mut guess_input = String::new();

    loop {

        println!("Please input your guess: ");

        io::stdin()
            .read_line(&mut guess_input)
            .expect("Failed to read line");

        let guess: u32 = match guess_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            },
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                guess_input.clear();
            },
            Ordering::Greater => {
                println!("Too big!");
                guess_input.clear()
            },
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }

    
}
