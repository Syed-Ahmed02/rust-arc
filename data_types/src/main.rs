//Datatypes 

// Every value in rust is a certain data type
// Rust is a statically typed langauge which means we must know all the variables at compile time

// The compiler can usually infer what type we want to use based on the value and how we use it
// However sometimes in cases where many types are possible such as when we converted String to numeric using parse in guessing game
// we must add a type annotation

// let guess: u32 = "42".parse().expect("Not a number!");

// If we don't add the u32 the compiler will give us an error
// There are two main type subsets, Scalar and Compound

//Scalar types
//Scalar types represent a single value, there is 4 main types, integers, floating point numbers, Booleans and Cahracters

//Integer types
// A number without a fractional componenet. We used one earlier in the guessing game "u32"
// The type declaration indicates that the value its associated with should be a unisgned or or signed 
// "i" for signed (can represnet both positive, negative and whole) and "u" for unsigned (can represent only non negative numbers) 
//Length	Signed	Unsigned
// 8-bit	i8	    u8
// 16-bit	i16	    u16
// 32-bit	i32	    u32
// 64-bit	i64	    u64
// 128-bit	i128	u128
// architecture dependent	isize	usize

// Each signed varient can store numbers from -(2^n-1) to 2^(n-1) -1 inclusive where n is the number of bits
// so for example i8 can store from -(2^7) to 2^7 -1 

//unsigned varients can store from 0 to 2^n - 1 so u8 can store 0 to 2^8 -1 

//isize and usize types depend on the architecture of the computer

// You can write integer literals in any of the forms below
// Number literals	Example
// Decimal	98_222
// Hex	0xff
// Octal	0o77
// Binary	0b1111_0000
// Byte (u8 only)	b'A'

// Integer overflow Error
// Lets say you have a variable of type u8 that can hold values between 0 and 255
// If you try to change the value to 256 you will get an error
// If it happens at runtime during debug mode rust will enter panic mode and exit the program with an error 


//Floating point types
// Floating point numbers are numbers with decimial points
// Rust's floating point types are f32 and f64 which are 32-bits and 64-bits respectively

// the default type is f64 because its roughly the same speed as f32 but more precise on modern cpus
// fn main() {
//     let x = 2.0; // f64

//     let y: f32 = 3.0; // f32
// }


// Numeric Operations
// Rusts supports basic mathematical operations such as addition, subtraction, multiplication and devision
// as well as integer division 
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}
// each expression in these statements uses a methimatical operator and evaluvates to a single value
// which is then bounded to a variable

// The Boolean type
// boolean values can be true and false and are 1 byte of sizes
// fn main() {
//     let t = true;

//     let f: bool = false; // with explicit type annotation
// }


//The Character type
// The char type is the most primitive alphabetic type. Here are some examples
// fn main() {
//     let c = 'z';
//     let z: char = 'ℤ'; // with explicit type annotation
//     let heart_eyed_cat = '😻';
// }

// Char type is 4 bytes in size and represnets a unicode sclar value which allows it to represent a lot more than just ASCII
// Unicode sclar values range from U+0000 to U+D7FF



//Compound types
// Compound types can group multipel values into one type, Rust has two primitive compound types
// Tuples and Arrays

// The tuple type

// A tuple is a general way of grouping together a number of vlaues with a variety of types into one compound type
// tuples have a fixed length, once declared they cannot grow or shrink in size

// a tuple can be declared by writing a comma-seperated list of values inside brackets.
// Each position in the tuple has a type and they can be different types, here is an example

// fn main() {
//     let tup: (i32, f64, u8) = (500, 6.4, 1);
// }

// the variable tup binds the entire tuple because a tuple is considered a single compound element
// To get the individual values out of the tuple we can use pattern matching to destructive tuple like this

// fn main() {
//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of y is: {y}");
// }

// We can also access a tuple element directly by using a period followed by the index we want to access

// fn main() {
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;

//     let six_point_four = x.1;

//     let one = x.2;
// }

// A tuple without any values has a special name, its called a unit
// This value and its corresponding type are both written () and represent an empty value or return an empty type



// The Array type
// Another way to have a collection of multiple values is with an array
// Unlike a tuple every elment of an array must have the same type
// In rust, Arrays have a fixed length

// fn main() {
//     let a = [1, 2, 3, 4, 5];
// }

// Arrays are useful when you want data allocated on the stack, the same as other types we have seen so far rather than on the heap.
// Its also useful when you want to ensure you always have a fixed number of elements
// An array isn't as flexiable as the vector type though. 
// A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size because the contents live on the heap

// However, arrays are more useful when you know the number of elements will not need to change
// for example
// let months = ["January", "February", "March", "April", "May", "June", "July",
//               "August", "September", "October", "November", "December"];

// You can write an arrays type in square bracket and the type of each element like this
//let a: [i32; 5] = [1, 2, 3, 4, 5];

// You can also initialize an array to contain the same value for each element by specifying the initial value followed by a semi colon
//let a = [3; 5];
// this is the same as writing
//let a = [3, 3, 3, 3, 3];

// An array is a single chunk of memory of a knwon fixed size that can be allocated on the stack
// you can access elements of an array using indexing like this

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     let first = a[0];
//     let second = a[1];
// }

// If you try to acess an element that is past the end of an array you will get an error at runtime


// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!("The value of the element at index {index} is: {element}");
// }

// for this code if you enter 0-4 the program will print the value inside the array at the index
// However if you enter an index that is past the end of the array such as 10 you will see an output like this
// thread 'main' panicked at src/main.rs:19:19:
// index out of bounds: the len is 5 but the index is 10
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

//The program resulted in a runtime error at the point of using invalid value in indexing operation
// The program ended with an error 
// This check will happen at run time because the compiler donesn't know what value a user will enter

// This is an example of rust's memory safety principles in action. In many low level languages this check is not done
// and when you try access an incorrect index, invalid memory can be accessed. 
// Rust protects you by immeditately exiting instead of allowing the moemory access to continue