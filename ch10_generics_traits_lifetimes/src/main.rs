use ch10_generics_traits_lifetimes::generics::{self, PointMixed};
use ch10_generics_traits_lifetimes::lifetimes;
use ch10_generics_traits_lifetimes::traits::{self, Summary};

fn main() {
    // Generics
    let number_list: Vec<i32> = vec![34, 50, 25, 100, 65];
    println!("largest number is {}", generics::largest_i32(&number_list));
    println!("largest number is {}", generics::largest(&number_list));

    let char_list: Vec<char> = vec!['y', 'm', 'a', 'q'];
    println!("The largest char is {}", generics::largest_char(&char_list));
    println!("The largest char is {}", generics::largest(&char_list));

    let integer: generics::Point<i32> = generics::Point::point(5, 10);
    let float: generics::Point<f32> = generics::Point::point(1.0, 4.0);

    println!("p.x = {}", integer.get_x());
    println!("p.x = {}", float.get_x());

    println!(
        "p is {} units away from the origin",
        float.distance_from_origin()
    );
    // Won't work because 'distance_from_origin' is only implemented for the type Point<f32>
    // println!("p is {} units away from the origin", integer.distance_from_origin());

    let p1: generics::PointMixed<i32, i32> = generics::PointMixed { x: 1, y: 2 };
    let p2: generics::PointMixed<f32, f32> = generics::PointMixed { x: 3.0, y: 4.0 };

    let p3: PointMixed<i32, f32> = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Traits
    let tweet: traits::Tweet = traits::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let book: traits::Book = traits::Book {
        author: "Isaac Newton".to_string(),
        title: "Principia Mathematica".to_string(),
    };

    println!("{}", book.summarize());

    // Lifetimes
    // This works
    let string1: String = String::from("abcd");
    let string2: &str = "xyz";

    let result: &str = lifetimes::longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // This also works
    let string1: String = String::from("long string is long");

    {
        let string2: String = String::from("xyz");
        let result: &str = lifetimes::longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // Does not work as string2 does not live long enough
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = lifetimes::longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let _i: lifetimes::ImportantExcerpt<'_> = lifetimes::ImportantExcerpt {
        part: first_sentence,
    };

    let s: &'static str = "I have a static lifetime.";
}
