use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    // Get range (1:100) number.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The Secret Number is : {secret_number}");

    // loop until you get the secret number.
    loop {
        println!("Please input your guess:");

        // setup guess as string.
        let mut guess = String::new();

        // Get input the number.
        io::stdin()
            // Get input number.
            .read_line(&mut guess)
            // If you get wrong, you will  get message.
            .expect("Failed to read line");

        // parse string to integer. If input non-number, you have to try it again.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // compare twwo numbers.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!1"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }   
}
