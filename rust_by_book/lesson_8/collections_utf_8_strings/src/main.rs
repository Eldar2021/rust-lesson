fn main() {
    let s1 = String::new();
    println!("s1: {}", s1);

    let s2s = "Hello 1";
    let s2 = s2s.to_string();
    println!("s2: {}", s2);

    let s3 = String::from("Hello 2");
    println!("s2: {}", s3);

    let hello1 = String::from("السلام عليكم");
    let hello2 = String::from("Dobrý den");
    let hello3 = String::from("Hello");
    let hello4 = String::from("שלום");
    let hello5 = String::from("नमस्ते");
    let hello6 = String::from("こんにちは");
    let hello7 = String::from("안녕하세요");
    let hello8 = String::from("你好");
    let hello9 = String::from("Olá");
    let hello10 = String::from("Здравствуйте");
    let hello11 = String::from("Hola");
    println!("{}", hello1);
    println!("{}", hello2);
    println!("{}", hello3);
    println!("{}", hello4);
    println!("{}", hello5);
    println!("{}", hello6);
    println!("{}", hello7);
    println!("{}", hello8);
    println!("{}", hello9);
    println!("{}", hello10);
    println!("{}", hello11);

    let mut s4 = String::from("foo");
    s4.push_str("bar");
    println!("{}", s4);

    let mut s5 = String::from("E");
    s5.push('l');
    s5.push('d');
    s5.push('i');
    s5.push('i');
    s5.push('a');
    s5.push('r');
    println!("{}", s5);

    let s6v1 = String::from("Eldi");
    let s6v2 = String::from("iar");
    let s6 = s6v1 + &s6v2;
    println!("{}", s6);

    let s7 = String::from("السلام عليكم");
    let s7v1 = s7.as_bytes();
    println!("{:?}", s7v1);
    let s7v2 = s7.chars().collect::<Vec<char>>();
    println!("{:?}", s7v2);

    let s8 = "Здравствуйте";
    let s8v1 = &s8[0..10];
    println!("{}", s8v1);
    let s8v2 = s8.as_bytes();
    println!("{:?}", s8v2);
}
