// What is Ownership

// Ownership is a set of rules that govern how a rust program manages memory
// Some languages have garbage collectors, others you have to allocate and free the memory
// Rust uses a different approach, in rust, memory is managed through a system of ownership with a set of rules that the compiler checks.
// If any of the rules are violaoted, the program won't compile. None of the features of ownership slow down your program while its running

// The Stack and Heap

//Stack
// Both the stack and heap are parts of memory that are avaliable to your code to use at runtime, but they are strucutred in different ways.
// The stack stores values in the order it gets them and removes in the opposite order, this is known as last in, first out
// adding = Pushing on to the stack, removing = popingg of the stack
// All data stored on the stack must have a known fixed size, data with an unknown size at compile time or size that might change must be stored on the heap

//Heap

fn main() {
    println!("Hello, world!");
}
