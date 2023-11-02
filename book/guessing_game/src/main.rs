use std::io;

fn main() {
    println!("Welcome to Guess the Number!\n");
    println!("Please enter your guess: ");
    
    let mut guess =  String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read a line");
    
    // also above three lines could be:
    // io::stdin().read_line(&mut guess).expect("dadada");

    println!("Your guessed: {guess}");


}
