fn main() {
    let mut vec1 = Vec::new();
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    // vec1.push(12.3); // HATA: vec1 is Vec of i32
    println!("vec1: {vec1:#?}");
    vec1.pop();
    println!("vec1: {vec1:#?}");

    let vec2 = vec![11, 12, 13];
    println!("vec2: {vec2:#?}");
    // vec2.push(14);  // HATA: vec2 is not mutable

    let vec3 = vec![21, 22, 23];
    println!("vec3: {vec3:#?}");
    // vec3.pop(); // HATA: vec3 is not mutable

    let vec4 = vec![31, 32, 33];
    println!("vec4: {vec4:#?}");
    let v1: &i32 = &vec4[2];
    println!("v1: {}", v1);
    // let v2: &i32 = &vec4[3];
    // println!("v2: {}", v2); // thread 'main' panicked at src/main.rs:23:25:
    let v3: Option<&i32> = vec4.get(3);
    match v3 {
        Some(v) => println!("v3: {}", v),
        None => println!("v3: none"),
    }

    // iterate
    let vec5 = vec![41, 42, 43];
    for v in &vec5 {
        println!("iterating immutable value v: {}", v);
    }

    let mut vec6 = vec![51, 52, 53];
    for v in &mut vec6 {
        println!("iterating mutable value v: {}", v);
        *v += 10;
    }
    println!("vec6: {vec6:#?}");
}
