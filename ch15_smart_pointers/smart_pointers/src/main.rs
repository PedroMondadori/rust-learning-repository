use list::List::{self, Cons, Nil};

fn main() {
    let _l: List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
