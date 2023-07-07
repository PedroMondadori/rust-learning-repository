#![allow(unused)]

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn print_block_return_value() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("{y}");
}

fn five() -> i32 {
    return 5 as i32;
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}

fn main() {
    // print_labeled_measurement(5, 'h');

    // print_block_return_value();

    // println!("{}", five());

    let x = plus_one(5);
    println!("{x}");
}
