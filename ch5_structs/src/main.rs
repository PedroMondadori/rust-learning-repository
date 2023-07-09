#![allow(unused)]

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }

    fn square(size: u32) -> Self {
        return Self {
            width: size,
            height: size,
        };
    }
}

fn area_v1(width: u32, height: u32) -> u32 {
    return width * height;
}

fn area_v2(dimensions: (u32, u32)) -> u32 {
    return dimensions.0 * dimensions.1;
}

fn area_v3(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}

fn main() {
    // let mut user1 = User {
    //     active: true,
    //     username: String::from("someusername123"),
    //     email: String::from("someone@example.com"),
    //     sign_in_count: 1,
    // };

    // user1.email = String::from("anotheremail@example.com");

    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     ..user1
    // };

    // let black: Color = Color(0, 0, 0);
    // let origin: Point = Point(0, 0, 0);
    // let subject: AlwaysEqual = AlwaysEqual;

    // // area_v1
    // let width1: u32 = 30;
    // let height1: u32 = 50;

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area_v1(width1, height1)
    // );

    // let rect1: (u32, u32) = (30, 50);

    // // area_v2
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area_v2(rect1)
    // );

    // // area_v3
    // let rect2: Rectangle = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area_v3(&rect2)
    // );

    // println!("rect2 is {:#?}", rect2);
    // dbg!(&rect2);

    // let rect3: Rectangle = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // println!(
    //     "The are of the rectangle is {} square pixels.",
    //     rect3.area()
    // );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square: Rectangle = Rectangle::square(3);
    dbg!(square);
}
