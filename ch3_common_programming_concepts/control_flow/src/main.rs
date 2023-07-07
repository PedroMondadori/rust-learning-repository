#![allow(unused)]

fn if_statement(number: i32, condition: i32) {
    if number < condition {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn repetition() {
    let mut counter: i32 = 0;
    
    let result: i32 = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;
        }
    };
    
    println!("The result is {result}");
}

fn break_label() {
    let mut count: i32 = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining: i32 = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn loop_collection() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn main() {
    // if_statement(5, 3);
    // repetition();
    // break_label();
    loop_collection();
}