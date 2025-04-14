pub trait State {
    fn send_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, document: &'a Document) -> &'a str;
}

pub struct Draft;

impl State for Draft {
    fn send_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(Review)
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, _document: &'a Document) -> &'a str {
        ""
    }
}

pub struct Review;

impl State for Review {
    fn send_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published)
    }

    fn content<'a>(&self, _document: &'a Document) -> &'a str {
        ""
    }
}

pub struct Published;

impl State for Published {
    fn send_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, document: &'a Document) -> &'a str {
        &document.content
    }
}

pub struct Document {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Document {
    pub fn new(content: String) -> Self {
        Document {
            state: Some(Box::new(Draft)),
            content: content,
        }
    }

    pub fn send_review(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.send_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve())
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
}
