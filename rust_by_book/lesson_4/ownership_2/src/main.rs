fn main() {
    // Reference
    let x = 10;  // `x` değişkeni oluşturulur
    let x_ref = &x;  // `x`'e bir referans alınır
    println!("x: {}", x);  // Orijinal `x` hâlâ erişilebilir
    println!("x_ref: {}", x_ref); // Referans ile değerine erişilir


    // Borrowing
    let s = String::from("Hello");
    let r1 = &s; // `s`'in immutable referansı
    let r2 = &s; // İkinci bir immutable referans alınabilir
    println!("r1: {}, r2: {}", r1, r2); // Her ikisi de kullanılabilir

    // Mutable Borrowing
    let mut s = String::from("Hello");
    let r = &mut s; // `s`'in mutable referansı alınır
    r.push_str(", World!"); // Mutable referans ile değişiklik yapılabilir
    println!("r {}", r);
    println!("s {}", s);

    // Lifetime (Ömür) ve Borrowing
    // let r;
    {
        let x = 5;
        println!("{}", x);
        // r = &x; // HATA: `x` bu bloğun sonunda yok edilir
    }
    // println!("{}", r); // Geçersiz bir referans kullanımı
}
