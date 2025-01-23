pub trait Summary {
    fn summarize(&self) -> String;

    fn min_summary(&self) -> String {
        String::from("Read more...")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("username: {}, content: {}", self.username, self.content)
    }
}

pub struct Article {
    pub title: String,
    pub content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("title: {}, content: {}", self.title, self.content)
    }

    fn min_summary(&self) -> String {
        String::from("Read more interesting Artilce...")
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

pub fn collect_two_summaries<T: Summary>(a: &T, b: &T) {
    println!("Collected two news! {}-{}", a.summarize(), b.summarize());
}
