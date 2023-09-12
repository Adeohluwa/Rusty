use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    loop {
        // secret
        let secret_number = rand::thread_rng().gen_range(1..=100);
        let mut guess = String::new();
       
        println!("Please input your Guess!");
        
        // accept input and bind to guess var, also handle error
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");       
    
        // sanitize guess before comparison
        let guess: u32 = guess.trim().parse().expect("Please type a number!");    
        println!("You guessed {guess}");
        println!("The secret number is {secret_number}");
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too big!"),
            Ordering::Equal => println!("you win!"),
        }
    }

}