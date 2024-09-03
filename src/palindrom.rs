use std::io;

fn is_palindrome(n: i32) -> bool {
    // Kenar durumu: Negatif sayılar palindromik olamaz
    if n < 0 {
        return false;
    }

    // Orijinal sayıyı sakla
    let original = n;
    let mut reversed = 0;
    let mut num = n;

    // Sayıyı ters çevir
    while num > 0 {
        let digit = num % 10;  // Son basamağı al
        reversed = reversed * 10 + digit;  // Ters çevrilmiş sayıya ekle
        num /= 10;  // Son basamağı çıkar
    }

    // Orijinal sayıyı ters çevrilmiş sayıyla karşılaştır
    if original == reversed {
        true
    } else {
        false
    }
}

fn main() {
    // Kullanıcıdan sayı girmesini iste
    println!("Bir sayı girin:");

    // Giriş verisini almak için değişken tanımla
    let mut input = String::new();

    // Kullanıcının girdiği veriyi oku
    io::stdin().read_line(&mut input).expect("Girdi okunamadı");

    // Girdiyi i32 türüne çevir
    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Geçersiz giriş!");
            return;
        }
    };

    // Palindrom olup olmadığını kontrol et
    let result = is_palindrome(number);

    // Sonucu ekrana yazdır
    if result {
        println!("Girdiğiniz sayı palindromiktir.");
    } else {
        println!("Girdiğiniz sayı palindromik değildir.");
    }
}