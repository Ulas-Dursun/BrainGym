use std::io;

fn main() {
    // Kullanıcıdan dizi elemanlarını al
    let mut input = String::new();
    println!("Dizi elemanlarını boşluklarla ayırarak girin:");
    io::stdin().read_line(&mut input).expect("Satır okunamadı");

    // Girdiyi tam sayılar vektörüne çevir
    let elements: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Dönüşüm hatası"))
        .collect();

    // Kullanıcıdan kaç eleman çıkartılacağını al
    let mut input = String::new();
    println!("Çıkartılacak eleman sayısını girin:");
    io::stdin().read_line(&mut input).expect("Satır okunamadı");
    let num_to_remove: usize = input.trim().parse().expect("Dönüşüm hatası");

    // Çıkartılacak eleman sayısının dizinin uzunluğunu aşmadığından emin ol
    let num_to_remove = num_to_remove.min(elements.len());

    // Yeni boyutlu bir dizi oluştur
    let reduced_elements: Vec<i32> = elements
        .iter()                      // Orijinal diziyi iteratörle gez
        .take(elements.len() - num_to_remove) // Yeni boyuta kadar elemanları al
        .copied()                    // Elemanları kopyala
        .collect();                  // Yeni bir vektöre topla

    // Azaltılmış diziyi ekrana yazdır
    println!("Azaltılmış dizi: {:?}", reduced_elements);
}