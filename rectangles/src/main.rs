// 5.4 an example program using structs 

// fn main() {
//     let width1 = 30;
//     let height1 = 50;

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(width1,height1)
//     );
// }

// fn area(width:u32,height:u32) -> u32{
//     width*height
// }

// This code works but the function we wrote (area) is supposed to calculate the area of one rectangle
// and its not clear that the two parameters (width and height) are related
// it would be more managable if we group width and height together. 
// One way to do that is using tuples


// fn main() {
//     let rectangle_1 = (30,50);

//     println!(
//         "The area of the rectangle is {} square pixels.",
//         area(rectangle_1)
//     );
// }

// fn area(dimensions: (u32,u32)) -> u32{
//     dimensions.0 * dimensions.1
// }

// In one way this is more clear, because we're passing one argument, in another way its more unclear
// because tuples don't let us name our elements so we have to index parts of the tuple making the calc unclear
// mixing up the width and height wouldn't matter but if we had to draw parts on the screen it would

// we can refactor this with structs 

struct Rectangle {
    width:u32,
    height:u32,
}


fn main() {
    let rectangle_1 = Rectangle {
        width:30,
        height:50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        //allows fn area to borrow the struct instead of owning it
        area(&rectangle_1)
    );
}

fn area(dimensions: &Rectangle) -> u32{
    dimensions.width * dimensions.height
}


//What if we want to print out debuging information (like the value of the struct)
// We can't simply print the struct as rust will throw an error
// thankfully rust does include functionality to allow us to debug using the #[derive(Debug)] right before the struct definition
// here is an example 

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// fn main() {
//     let rect1 = Rectangle {
//         width: 30,
//         height: 50,
//     };

//     println!("rect1 is {rect1:?}");
// }