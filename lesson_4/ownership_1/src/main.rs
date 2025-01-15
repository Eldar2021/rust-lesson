fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // Sahiplik `s1`'den `s2`'ye taşınır.
    // println!("{}", s1); // HATA: `s1` artık geçerli değil!
    println!("{}", s2); // `s2` geçerlidir ve veri buradan erişilir.

    let s3 = String::from("Hello 1");
    let s4 = s3.clone(); // Sahiplik `s3`'den `s4`'ye taşınır.
    println!("{}", s3); // `s3` geçerlidir ve veri buradan erişilir.
    println!("{}", s4); // `s4` geçerlidir ve veri buradan erişilir.

    let s5 = String::from("Hello 2");
    let len = calculate_length(&s5); // Sahiplik `s5`'den `len`'e taşınır.
    println!("The length of '{}' is {}.", s5, len);

    let mut s6 = String::from("Hello 3");
    println!("{}", s6);
    change_value(&mut s6);
    println!("{}", s6);

    let s7 = String::from("Hello"); // `String` heap'te saklanır.
    println!("The length of '{}' is {}.", s7, s7.len());
}

fn calculate_length(s: &String) -> usize {
    s.len() // `s` sadece okunabilir bir referans.
}

fn change_value(s: &mut String) {
    s.push_str(", world"); // Veri mutable referansla değiştirilebilir.
}