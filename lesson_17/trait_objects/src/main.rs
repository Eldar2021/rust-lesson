use trait_objects::{Button, Screen, SelectBox};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 12,
                height: 12,
                options: vec![String::from("Test 1")],
            }),
            Box::new(Button {
                width: 34,
                height: 52,
                label: String::from("Click!"),
            }),
        ],
    };

    screen.run();
}
