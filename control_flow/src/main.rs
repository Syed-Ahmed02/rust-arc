// the ability to run code if some statement is true is common in most progrmaing language 

// if statements
// if statements allows you to branch your code based on a condition.

// fn main() {
//     let number = 3;
//     // if number is < 5 then run this code
//     if number < 5 {
//         println!("condition was true");
//     } else { // otherwise run this code
//         println!("condition was false");
//     }
// }


// the condition in the code mnust be a bool, if the condiiton isn't a bool we will get an error
// for example
// fn main() {
//     let number = 3;

//     if number {
//         println!("number was three");
//     }
// }

// unlike js and ruby Rust will not automatically try to convert the statement into a bool
// we can add multiple conditions using else if
// fn main() {
//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// because if is an expression we can use it on the right side of a let statement to assign the outcome to a var

// fn main() {
//     let condition = true;
//     let number if condition {5} else {6}; // if condition is true assign to 5 otherwise awssign to 6
//     println!("The value of number is: {number}");
// }

// blocks of code evaluvate to the last expression in them, numbers by themselves are also expresions
// . In this case, the value of the whole if expression depends on which block of code executes. This means the values that have the potential to be results from each arm of the if must be the same type
// // if the types are mismatched we get an error 
// fn main() {
//     let condition = true;

//     let number = if condition { 5 } else { "six" };

//     println!("The value of number is: {number}");
// }


// Repitition with loops
// Rust provides serveral ways to execute a block of code more than once

// Rust has three kinds of loops; loop, while and for

//Repeating code with Loop
// The loop keyword tells rust to excute a code block forever or untill you tell it to stop

//Example
// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// This code will run forever untill we stop the program manually using ctrl + c
//  You can also break out the code using the "break" keyword within the loop to tell the program to stop executing the loop

//Returning values from loops
// one of the uses of loop is to retry an operation you know might fail such as checking wheather a thread has completed its jobs
// You might need to pass the result, to do this add the value you want returned after the break statement

//Example
// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

// You can aslo "return" from inside the loop, while break only exits the current loop, return always exits the current function

// Loop Labels to Disambiguate Between Multiple Loops
// If you have loops within loops, break and continue apply to the innermost loop at that point
// You can optionally specify a loop label that you can use with break or continue 

//Example
// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }

// The output 
// count = 0
// remaining = 10
// remaining = 9
// count = 1
// remaining = 10
// remaining = 9
// count = 2
// remaining = 10
// End count = 2

//Conditional Loops with While
// We use conditional loops when we want to repeat code when a condition is true 
//Example
// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

//Looping through a collection with for
// Lets say you want to loop through the elements in an array, you can use the for keyword to do it
// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a {
//         println!("the value is: {element}");
//     }
// }

// This code is more safe then the while loop version as it eliminated the chance of trying to index a value that doesn't exist in an array

// Even in situations you want to run some code a certain amount of times such as in the countdown example for while loop
// YOu can use a floop loop using the Range provided by the standard library

fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
// .rev reverses the range