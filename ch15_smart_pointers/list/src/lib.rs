// pub struct pair {
//     fst: i32,
//     snd: Box<pair>,
// }

pub enum List {
    Cons(i32, Box<List>),
    Nil,
}



