use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let maxnum = 101;
    let mut guesscount = 0;
    println!("Guess the number (0 - {})!\n", maxnum - 1);

    let secret_number = rand::thread_rng().gen_range(0,101);

    //println!("The secret number is {}\n", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        guesscount = guesscount + 1;
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
            

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("\nYou win!\nIt only took you {} guesses\n\n", guesscount);
                break;
            }
        }
    }
}
