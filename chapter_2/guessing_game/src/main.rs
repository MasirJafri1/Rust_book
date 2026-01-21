use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number game by MasirJafri");

    let secret_number: u32 = rand::rng().random_range(1..=100);
    // println!("The secret number is {secret_number}");

    loop {
        println!("Please input a guess : ");

        let mut guess: String = String::new();

        // guess.push_str("My name is MasirJafri");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read user input");

        let guess: u32 =match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number !");
                continue;
            }
        }; // Shadowing

        println!("You guessed : {guess} ");

        match guess.cmp(&secret_number) {
            //it is exhaustive so every case must be handled
            Ordering::Equal => {
                println!("You win !");
                break;
            },
            Ordering::Less => println!("Too small !"),
            Ordering::Greater => println!("Too big !"),
        }
    }
}
