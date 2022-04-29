use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the Number Game");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess");
    
        let mut guess = String::new();
        io::stdin().read_line(& mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal =>{
                 println!("Correct!");
                 break;
            },
            Ordering::Greater => println!("Too bigggg!"),
        }
    }

}
