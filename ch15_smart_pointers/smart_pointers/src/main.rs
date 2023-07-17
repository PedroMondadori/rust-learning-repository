use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use list::List::{self, Cons, Nil};
use references::{
    coercion_deref,
    custom_smart_pointers::{CustomSmartPointer, MyBox},
    cycles::{
        CList::{self, CCons, CNil},
        Node,
    },
    mutability_pattern::MPList::{self, MPCons, MPNil},
    reference_counting::RCList::{self, RCCons, RCNil},
};

fn main() {
    //
    // list
    //
    let _l: List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    println!();
    //
    // custom_smart_pointers
    //
    let m = MyBox::new(String::from("Rust"));
    coercion_deref(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    drop(c);
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    println!();
    //
    // reference_counting
    //
    let a: Rc<RCList> = Rc::new(RCCons(5, Rc::new(RCCons(10, Rc::new(RCNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b: RCList = RCCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c: RCList = RCCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    println!();
    //
    // mutability_pattern
    //
    let value: Rc<RefCell<i32>> = Rc::new(RefCell::new(5));

    let a: Rc<MPList> = Rc::new(MPCons(Rc::clone(&value), Rc::new(MPNil)));

    let b: MPList = MPCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c: MPList = MPCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    println!();
    //
    // cycles
    //
    let a: Rc<CList> = Rc::new(CCons(5, RefCell::new(Rc::new(CNil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b: Rc<CList> = Rc::new(CCons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    println!();
    //
    // Graph example
    //
    let leaf: Rc<Node> = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    {
        let branch: Rc<Node> = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}
