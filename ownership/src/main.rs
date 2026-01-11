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
// The heap is less organized, when you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough
// and marks it as being used. This is called allocating on the heap
// accessing data on the heap is generally slower than accessing data on the stack because you have to follow a pointer to get there

// Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data and cleaning up unnused data on the heap are all problems ownership addresses


// Ownership Rules

// Each value in Rust has an owner.
// There can only be one owner at a time.
// When the owner goes out of scope, the value will be dropped.

// Variable Scope
// The scope refers to the range within your program for which the value is valid
// In the following example you can see the scope of s

// fn main()     {                      // s is not valid here, since it's not yet declared
// let s = "hello";   // s is valid from this point forward

// // do stuff with s
// }                      // this scope is now over, and s is no longer valid

// The string type

// 
