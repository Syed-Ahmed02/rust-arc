// when a variable is immutable it cannot be changed after it is bound to a name
// by default variables are immutable
// fn main() {

//     //This code gives an error 
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }


// the keyword mut allows you to make variables mutable 

//constants are simmilar to immutable values with a few key diffrences
// Constants are always immutable you can't use mut 
// you declare constants with const instead of let 
// the type and value must be annotated 
// constants can be declared in any scope, including the global scope 

//const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// rust naming convention is to use all capital with underscares between words for constants

// constants are valid for the entire time they run not only the scope they are declared

