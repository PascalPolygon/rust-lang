use std::io;
use rand::Rng; //Rng is a trait, not a library
use std::cmp::Ordering;

fn main() {
    println!("Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1, 101); //generates from 1 to 100

    // println!("Secret number: {}", secret_number);

    loop{ //infinite loop
        println!("Please enter your guess");

        let mut guess = String::new(); //mut is keyword for a mutable variable

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        //trim removes all whitespaces and newlike chars from string and parse converts to a number
        let guess: u32 = match guess.trim().parse() { // converting string to unsigned 32 bit number for comparison. also uses shadowing which lets me name 2 variables the same thing in case of type conversion like here
            Ok(num) => num, //if input valid num
            Err(_) => continue, //if invalid input underscore is catch all errors, program continues to next loop iteration
        };


        println!("You guessed {}", guess);


        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big! (That's what she said ;) )"),
            Ordering::Equal => {
                println!("Perfect match!");
                break; //will break out of the loop and the program because nothing comes next
            }
        }
    }

}
