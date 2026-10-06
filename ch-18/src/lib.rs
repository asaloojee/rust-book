pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
    num_of_approvals: i32,
}

impl Default for Post {
    fn default() -> Self {
        Self::new()
    }
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
            num_of_approvals: 0,
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve(&mut self.num_of_approvals))
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject(&mut self.num_of_approvals))
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>, num_of_approvals: &mut i32) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn reject(self: Box<Self>, num_of_approvals: &mut i32) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>, _num_of_approvals: &mut i32) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>, _num_of_approvals: &mut i32) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>, num_of_approvals: &mut i32) -> Box<dyn State> {
        if *num_of_approvals == 0 {
            *num_of_approvals += 1;
            Box::new(PendingReview {})
        } else if *num_of_approvals == 1 {
            *num_of_approvals += 1;
            Box::new(Published {})
        } else {
            *num_of_approvals = 0;
            Box::new(Draft {})
        }
    }

    fn reject(self: Box<Self>, num_of_approvals: &mut i32) -> Box<dyn State> {
        *num_of_approvals = 0;
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>, _num_of_approvals: &mut i32) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }

    fn reject(self: Box<Self>, _num_of_approvals: &mut i32) -> Box<dyn State> {
        self
    }
}
