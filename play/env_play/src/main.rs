// env in rust 🤔
// https://crates.io/crates/dotenv
/*
The easiest and most common usage consists on calling `dotenv::dotenv`
when the application starts, which will load environment variables from 
a file named .env in the current directory or any of its parents; 
after that, you can just call the environment-related method you need as 
provided by `std::os`.
*/



extern crate dotenv;
// use dotenv::dotenv;
use std::env;

fn main() {
    println!("Reading env stuff");
    dotenv::dotenv().ok();

    // for (key, value) in env::vars() {
    //     println!("{}: {}", key, value);
    // }

    // https://doc.rust-lang.org/std/env/fn.var.html
    // println!("{:?}", env::var("PLAY_NAME"));

    // proper way to see if our env var exists
    match env::var("PLAY_NAME") {
        Ok(val) => println!("found our key: {val}"),
        Err(e) => println!("that key wasn't found: {e}")
    }

}
