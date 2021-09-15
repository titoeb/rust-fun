pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: String) {
        if let Some(s) = &self.state {
            if let Some(text) = s.add_text(text){
                self.content.push_str(&text);
            }
        }
    }
    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn add_text(&self,  _text: String) -> Option<String>{
        None
    }
}

struct Draft {}


impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {approve_count: 0})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State>{
        self
    }
    fn add_text(&self,  _text: String) -> Option<String>{
        Some(_text)
    }
}

struct PendingReview {
    approve_count: i32
}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        match self.approve_count{
            0 => {
                self.approve_count += 1;
                self
            }
            1 => {
                Box::new(Published {})
            }
            _ => {
                panic!("PendingReview.approve_count={} is undefined behaviour.", self.approve_count)
            }
        }
    }
    fn reject(self: Box<Self>) -> Box<dyn State>{
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
    fn reject(self: Box<Self>) -> Box<dyn State>{
        self
    }
}

fn main() {
    let mut post = Post::new();

    post.add_text(String::from("I ate a salad for lunch today"));
    assert_eq!("", post.content());

    post.request_review();
    post.add_text(String::from("Nothing added when PendingReview"));
    assert_eq!("", post.content());

    post.reject();
    post.add_text(String::from(". It was nice."));
    assert_eq!("", post.content());
    
    post.request_review();
    post.add_text(String::from("Nothing added when state is `PendingReview`"));
    assert_eq!("", post.content());
    
    post.approve();
    post.add_text(String::from("Nothing added when state is `Published`"));
    assert_eq!("", post.content());
    
    post.approve();
    assert_eq!("I ate a salad for lunch today. It was nice.", post.content());
}