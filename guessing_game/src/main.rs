// use std::io; //Input / output libarary in 
// // Other standard libaries found here: https://doc.rust-lang.org/std/prelude/index.html 


// fn main(){
    
//     println!("Guess the number!");
    
//     println!("Please input your guess.");


//     // the word "let" creates a variable, for example let apples = 5 means apples is 5
//     // mut means the variable is mutable 
//     // "=" binds the var name to the value
//     // "String" is a string type provided by the standard libaray 
//     // The "::new" means new is an associated function of the String type, the new function creates an empty string
//     // In full, the let mut guess = String::new(); line has created a mutable variable that is currently bound to a new, empty instance of a String    
//     let mut guess = String::new();

//     io::stdin()  //Allows us to handle input 
//         .read_line(&mut guess) // lets us get the input from the user and defines where to store it. "&" means that the arg is a refrence. Refrences are immutable by default which is why you need to do &mut 
//         .expect("Failed to read line"); //handles potential errors

//     //read_line puts whatever the user enters into a string and returns a Result value which is a enumeration, values are Ok and Err
//     // If you dont call "expect" the progrma will compile but you will get a warning 
//     println!("You guessed: {guess}");
//     // {} lets us print whatever variable is in the bracket
// }

// Part 2: Generating a secret number

// Need to add a crate "rand" to Cargo.toml
// In [dependencies] you tell Cargo which external crates your project depends on and which versions of those crates you require.
//Cargo.lock is a mechanism to ensure you can rebuild the same artifcat everytime you or someone else runs the code

// use std::io;
// use rand::Rng;

// fn main() {
//     println!("Guess the number!");

//     // thread_rng() gives us a random number that is local to the current thread of execution and seeded by os
//     //gen_range takes a range expression as an arg and generates a random number from that range
//     // start..=end is inclusive of lower and upper bound 1..=100 requests a number from 1 - 100
//     let secret_number = rand::thread_rng().gen_range(1..=100); 

//     println!("The secret number is: {secret_number}");

//     println!("Please input your guess.");

//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//     println!("You guessed: {guess}");
// }


//part 3 comparing the guess to secret number


// use std::cmp::Ordering; // Ordering is another enum and has varients Less, Greater, and Equal. 
// use std::io;

// use rand::Rng;

// fn main() {
//     println!("Guess the number!");

//     let secret_number = rand::thread_rng().gen_range(1..=100); 
    
//     println!("The secret number is: {secret_number}");
    
//     let mut guess = String::new();

//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");
    
//     let guess: u32 = guess.trim().parse().expect("Please type a number!");
//     // Rust allows us to shadow the previous value of guess with a new one 
//     //Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables
//     //We bind this new variable to the expression guess.trim().parse().
//     // The guess in the expression refers to the original guess variable that contained the input as a string. 
//     //The trim method on a String instance will eliminate any whitespace at the beginning and end, which we must do before we can convert the string to a u32, which can only contain numerical data.
    
//     // The colon after guess tells rust we will annotate the var's type. u32 means unsigned 32 bit

//     println!("You guessed: {guess}");

//     // the .cmp method compares two values, it takes a refrence to whatever you want to compare
//     match guess.cmp(&secret_number) { 
//         //match expression is made out of arms, an Arm consists of a pattern to match against. 
//         // rust takes the value given to match and looks through each pattern in turn
//         Ordering::Less => println!("Too small!"), 
//         Ordering::Greater => println!("Too big!"),
//         Ordering::Equal => println!("You win!"),
//     }
// }


//part 5 Looping
use std::cmp::Ordering; 
use std::io;

use rand::Rng;
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); 
        


    loop { // Loop keyword  creates an infinite loop, ctrl c 
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        //handling invalid input
        let guess: u32 = match guess.trim().parse() {  // uses the match keyword to check if there is an error or not
            Ok(num) => num,
            Err(_) => continue, // The "_" is a catchall value
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // stops the loop when the user 
            }
        }
    }
}