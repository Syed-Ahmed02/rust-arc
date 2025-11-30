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
// if the types are mismatched we get an error 
// fn main() {
//     let condition = true;

//     let number = if condition { 5 } else { "six" };

//     println!("The value of number is: {number}");
// }
