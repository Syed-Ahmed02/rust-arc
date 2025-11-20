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


//Constants

// Constants are simmilar to immutable values with a few key diffrences
// Constants are always immutable you can't use mut 
// you declare constants with const instead of let 
// the type and value must be annotated 
// constants can be declared in any scope, including the global scope 

//const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
// rust naming convention is to use all capital with underscares between words for constants

// constants are valid for the entire time they run not only the scope they are declared


//Shadowing
// Shadowing is when you declare a new variable with the same name as the old one
// We say the first variable is shadowed by the second. Here isan example

fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

// The compiler will output 
// $ cargo run
//    Compiling variables v0.1.0 (file:///projects/variables)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
//      Running `target/debug/variables`
// The value of x in the inner scope is: 12
// The value of x is: 6

//Shadowing is different from making a var mut as we will get a cmopile time error if we accidently try reassinging the var without using the let keyword
//By using let we can apply a few transformations on the var and let it be mutable after

// The other difference between mut and shadowing is because we are creating a new var when we use let 
// When we use mut it modifies the xisting variable
