fn faktoriyel(n: i32) -> i32 {
    let mut sonuc = 1; // Başlangıç değeri 1, çünkü 0! ve 1! sonuçları 1'dir.

    // 2'den n'e kadar olan sayılarla çarpma işlemi yapar
    for i in 2..=n { // 2'den n'e kadar olan aralık
        sonuc = sonuc * i; // her adımda sonucu günceller
    }

    sonuc // Hesaplanan faktöriyeli döndürür
}

// Permütasyon hesaplayan fonksiyon
fn permutasyon(n: i32, r: i32) -> i32 {
    // Permütasyon formülü: P(n, r) = n! / (n - r)!
    faktoriyel(n) / faktoriyel(n - r) // n! ve (n-r)!'i hesaplayıp bölerek sonucu döndürür
}

fn main() {
    let n = 10; // Toplam öğe sayısı
    let r = 3;  // Seçim sayısı

    // Permütasyonu hesapla
    let sonuc = permutasyon(n, r);

    // Sonucu ekrana yazdır
    println!("Permütasyon sonucu: {}", sonuc); // Örnek çıktı: Permütasyon sonucu: 720
}