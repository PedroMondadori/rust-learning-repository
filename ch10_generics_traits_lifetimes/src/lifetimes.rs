use std::fmt::Display;

//
//     [LIFETIME ELISION RULES]
//
// 1 - The compiler assigns a lifetime parameter to each parameter that’s a reference.
//
// 2 - If there is exactly one input lifetime parameter, that lifetime is assigned to 
//     all output lifetime parameters
//
// 3 - If there are multiple input lifetime parameters, but one of them is &self or &mut 
//     self because this is a method, the lifetime of self is assigned to all output lifetime parameters.
//

// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }
pub struct ImportantExcerpt<'a> {
    pub part: &'a str,
}

#[allow(unused)]
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        return 3;
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[allow(unused)]
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
