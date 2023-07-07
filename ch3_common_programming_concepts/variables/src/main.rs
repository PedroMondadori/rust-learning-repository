// fn main() {
//     let mut x: i32 = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// fn main() {
//     let x: i32 = 5;

//     let x: i32 = x + 1;

//     {
//         let x: i32 = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x is: {x}");
// }

// I didn't exactly follow the lesson here...
// Got caught up in testing type casting and integer overflows
// fn main() {
//     let spaces: String = " ".repeat(256);
//     let spaces: u8 = spaces.len() as u8;

//     println!("The number of spaces is {spaces}");
// }

use std::io;

fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index: String = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element: i32 = a[index];

    println!("The value of the element at index {index} is: {element}");
}
