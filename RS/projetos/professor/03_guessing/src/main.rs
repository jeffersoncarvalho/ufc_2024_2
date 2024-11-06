use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess The Number Game!");
    //generating the secret number
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");

    loop {
        //asking for user guess
        println!("Please, input you guess: ");
        let mut guess = String::new();

        //reading the input line
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //testing if your guess is a number (shadowing)
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => { println!("Invalid number!"); continue },
        };

        //showing your guess
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => { 
                println!("You win!");
                break;
            },
        }
    }//end loop
}
