// Declarative Macros with macro_rules! for General Metaprogramming
#[macro_export]
macro_rules! custom_vec {
    ($($x:expr), *) => {{
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*

        temp_vec
    }};
}

pub fn fn_1() {
    let v1 = vec![1, 2, 3, 4, 5];
    println!("V1: {v1:?}");

    let v2 = custom_vec!(1, 2, 3, 4, 5, 6);
    println!("V2: {v2:?}");
}

/// Procedural Macros for Generating Code from Attributes
// #[some_attribute]
// pub fn some_name(input: TokenStream) -> TokenStream {
//     let _ = input;
// }

pub fn fn_2() {}
