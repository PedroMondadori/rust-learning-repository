use blog::Post;
use gui::Button;
use gui::Draw;
use gui::Screen;
use oop_example::AveragedCollection;

#[allow(unused)]
#[derive(Debug)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    //
    // oop_example
    //
    let mut values = AveragedCollection::new();
    values.add(1);
    values.add(2);
    values.add(3);

    println!("{:?}", values);

    values.remove();

    println!("{:?}", values);

    println!();
    //
    // gui
    //
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();

    //
    // blog
    //
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    post.add_text(" and I liked it.");
    assert_eq!("I ate a salad for lunch today", post.content());
}
