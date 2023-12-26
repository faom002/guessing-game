use std::io;
use std::cmp::Ordering;

use rand::Rng;

fn main() {

    println!("Hello, there guess the number!");

    let guess = rand::thread_rng().gen_range(1..=100);

   loop {
       
   
    println!("please inpur your guess");

    let mut answer = String::new();
        

    io::stdin()
        .read_line(&mut answer)
        .expect("failed to read line");


    let answer: u32 = match answer.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("you guessed: {answer}");

    match answer.cmp(&guess) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }
   }
}
