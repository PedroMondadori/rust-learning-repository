use list::List::{self, Cons, Nil};
use references::{MyBox, CustomSmartPointer, coercion_deref};

fn main() {
    let _l: List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let m = MyBox::new(String::from("Rust"));
    coercion_deref(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(c);
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
}
