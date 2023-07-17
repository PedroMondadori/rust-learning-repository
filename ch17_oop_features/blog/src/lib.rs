pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        return Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        };
    }

    pub fn add_text(&mut self, text: &str) {
        if let Some(s) = self.state.take() {
            let new_text = s.add_text(text);
            self.content.push_str(new_text);
            self.state = Some(s);
        }
    }

    pub fn content(&self) -> &str {
        return self.state.as_ref().unwrap().content(self);
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }
}

trait State {
    fn add_text<'a>(&'a self, text: &'a str) -> &'a str;
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    #[allow(unused)]
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        return "";
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return Box::new(PendingReview { approvals: 0 });
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    fn add_text<'a>(&'a self, text: &'a str) -> &'a str {
        return text;
    }
}

struct PendingReview {
    approvals: u32,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        if self.approvals == 1 {
            return Box::new(Published {});
        }

        return Box::new(PendingReview { approvals: 1 })
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        return Box::new(Draft {});
    }

    #[allow(unused)]
    fn add_text<'a>(&'a self, text: &'a str) -> &'a str {
        return "";
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        return &post.content;
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        return self;
    }

    #[allow(unused)]
    fn add_text<'a>(&'a self, text: &'a str) -> &'a str {
        return "";
    }
}
