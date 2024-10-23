use std::io;

fn main() {
    println!("Guess The Number Game!");
    println!("Please, input you guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {}", guess);    
}
