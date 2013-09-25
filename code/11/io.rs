use std::io;
use std::rand;

fn generate_secret_number() -> int { 
    (rand::random::<int>() % 100).abs() + 1
} 

fn process_guess(secret:int, guess: int) -> bool { 
    println(fmt!("You guessed: %d", guess)); 

    if guess > secret { 
        println("Your guess was too high!"); 
        false 
    } else if guess < secret { 
        println("Your guess was too low!"); 
        false 
    } else { 
        println("You got it!"); 
        true 
    } 
} 


fn main() { 
    let secret = generate_secret_number(); 

    println("--- N U M B E R - G A M E ---"); 
    println(""); 
    println("Guess a number from 1-100 (you get five tries):"); 

    for round in range(0, 5) { 
        println(fmt!("Guess #%d", round)); 

        let input = io::stdin().read_line(); 

        match from_str::<int>(input) { 
            Some(number) => { 
                if process_guess(secret, number) { break; } 
            } 
            None         => println("Hey, put in a number.") 
        } 
    } 

    println("Done!"); 
} 
