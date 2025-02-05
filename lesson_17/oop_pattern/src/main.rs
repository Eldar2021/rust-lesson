use oop_pattern::Document;

fn main() {
    let mut doc = Document::new(String::from("This is document content"));

    println!("Content is Draft state: {}", doc.content());

    doc.send_review();

    println!("Content is Review state: {}", doc.content());

    doc.approve();

    println!("Content is Publish state: {}", doc.content());

    println!("Hello, world!");
}
