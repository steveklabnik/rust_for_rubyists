use std::io;
use std::rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = std::rand::task_rng().gen_range(1i, 101);
    println!("Secret number is {}", secret_number);

    let max_number_of_tries = 5;
    let mut guesses: int = 0;
    let mut reader = io::stdin();

    loop {
        if guesses == max_number_of_tries {
          println!("You failed to guess within the limit of {:d} guesses!", max_number_of_tries);
          break;
        }
        println!("Please input guess number {}", guesses + 1);

        let input = reader.read_line().ok().expect("Failed to read line");
        let input_num: Option<int> = from_str(input.as_slice().trim());

        let num = match input_num  {
            Some(num) => num,
            None      => {
                println!("Please input a number.");
                continue;
            }
        };

        println!("You guessed: {}", num);
        guesses += 1;

        match num.cmp(&secret_number) {
            Less    => println!("Too small!"),
            Greater => println!("Too big!"),
            Equal   => {
                println!("You win!");
                println!("You took {} guesses!", guesses);
                break;
            },
        }
    }
}
