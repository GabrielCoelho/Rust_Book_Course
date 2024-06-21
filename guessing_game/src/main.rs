use ::std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=10);

    let mut guess = String::new();
    //let apples = 5; immutable
    //let mut bananas = 5; mutable

    loop {
        println!("Please guess a number: ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //shadowing to avoid mismatched types
        let guess: u32 = guess.trim().parse().expect("Please type a number");

        println!("You guessed {}", guess);
        //let x = 5;
        //let y = 10;
        //println!("x = {x} and y + 2 = {}", y+2);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
            Ordering::Greater => println!("Too big"),
        }
    }
}
