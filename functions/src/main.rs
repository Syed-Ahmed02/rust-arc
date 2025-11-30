//the main function is the entry point of many programs
// you can declare a new function using the "fn" keyword

//Rust uses snake case for var and function names

// fn main() {
//     println!("Hello, world!");
//     // calls "another_function"
//     another_function();
// }

// fn another_function() {
//     println!("Another function.");
// }


//Parameters
// When a function has a params you can provide it with concrete variables which are known as args
// fn main() {
//     another_function(5); // calls another_function but passes in 5 
// }
// //x:i32 defines the name and type of the function
// // in function signatures you must declare the type of each arg

// fn another_function(x: i32 ) {
//     println!("The value of x is: {x}");
// }

// statements and expressions
// function bodies are made up of statements optionally ending in an expression
//Statements are instructions that perform some action and do not return a value.
//Expressions evaluate to a resultant value.


// let y = 6 is an example of a statement 
// function definitions are also statement 
//statements don't return a value theirfore you cannot assign a let statement to another variable 
// this will give you an error
//fn main() {
//     let x = (let y = 6);
// }

// (let y = 6) doesn't return a variable so there is nthn for x to be binded too
// this is different from another languages such as c where assignment returns the value so you can do something like x = y = 6 in those langs and have both x & y = 6

// expressions evaluvate to a value 
// a math operation such as  5 + 6 is a expression
// expressions can be apart of statement for example the 6 in let x = 6 is a expression
// calling a function and and macro is an expression 

// 
// fn main() {
//     let y = {
//         let x = 3;
//         x + 1 // x + 1 gets boudned to y as part of the expresion 
//         // expressions don't have semicollons at the end, if you add a semi collon it becomes a statement 
//     };

//     println!("The value of y is: {y}");
// }


//Functions with return values
// functions can return values to the code that calls them
// we don't name return values but we must declare their type after an arrow (->)
// in rust the the return value of function is synonymous with the value of the final expression in the block of the body of a function


// -> i32 means the function will return an unsigned 32 bit integer value
fn five() -> i32  {
    5 // since 5 is the last expression it will be returned
}

// fn main() {
//     let x = five();

//     println!("The value of x is: {x}");
// }


// this is an another example with paramaters 
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1 // Adding a ; will result in an error because it will turn this into a statement
}


// The main error message, mismatched types, reveals the core issue with this code. The definition of the function plus_one says that it will return an i32, but statements don’t evaluate to a value, which is expressed by (), the unit type. Therefore, nothing is returned, which contradicts the function definition and results in an error. In this output, Rust provides a message to possibly help rectify this issue: it suggests removing the semicolon, which would fix the error.

