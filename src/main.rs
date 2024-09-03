mod en_kucuk_fark;
mod dizi_kucult;
mod palindrom;
mod matematik_islemler;
mod faktoriyel_permutasyon;

use std::io;
use en_kucuk_fark::en_kucuk_fark; // en_kucuk_fark işlevini kullan


fn main() {
    println!("1. En Küçük Fark:");
    let dizi1 = vec![1, 3, 15, 11, 2];
    let dizi2 = vec![23, 127, 235, 19, 8];
    let en_kucuk_fark = en_kucuk_fark::en_kucuk_fark(&dizi1, &dizi2);
    println!("İki dizi arasındaki en küçük fark: {}", en_kucuk_fark);

    println!("\n2. Dizi Küçültme:");
    let mut input = String::new();
    println!("Dizi elemanlarını boşluklarla ayırarak girin:");
    io::stdin().read_line(&mut input).expect("Satır okunamadı");
    let elements: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Dönüşüm hatası"))
        .collect();
    println!("Çıkartılacak eleman sayısını girin:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Satır okunamadı");
    let num_to_remove: usize = input.trim().parse().expect("Dönüşüm hatası");
    let reduced_elements = dizi_kucult::dizi_kucult(&elements, num_to_remove);
    println!("Azaltılmış dizi: {:?}", reduced_elements);

    println!("\n3. Palindrom Kontrolü:");
    println!("Bir sayı girin:");
    input.clear();
    io::stdin().read_line(&mut input).expect("Girdi okunamadı");
    let number: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Geçersiz giriş!");
            return;
        }
    };
    let is_palindromic = palindrom::is_palindrome(number);
    if is_palindromic {
        println!("Girdiğiniz sayı palindromiktir.");
    } else {
        println!("Girdiğiniz sayı palindromik değildir.");
    }

    println!("\n4. Matematiksel İfade Değerlendirme:");
    let ifade = "2*3+5/6*3+15";
    let sonuc = matematik_islemler::ifade_degerlendir(ifade);
    println!("Sonuç: {}", sonuc);

    println!("\n5. Faktöriyel ve Permütasyon:");
    let n = 10;
    let r = 3;
    let permutasyon_sonuc = faktoriyel_permutasyon::permutasyon(n, r);
    println!("Permütasyon sonucu: {}", permutasyon_sonuc);
}


/*fn en_kucuk_fark(dizi1: &[i32], dizi2: &[i32]) -> i32 {
    let mut en_kucuk_deger = i32::MAX; // En küçük farkı bulmak için başlangıçta maksimum değeri atıyoruz.

    // Her bir elemanın mutlak değerini hesaplamak için abs() fonksiyonunu kullanıyoruz.
    // Dizideki elemanların değerlerini eleman1 ve eleman2 ile doğrudan kullanarak,
    // dizideki elemanları birbirleriyle karşılaştırıp en küçük farkı bulup değişkenimize atayacağız.

    for &eleman1 in dizi1 { // İlk dizi içerisindeki her bir elemanı eleman1 olarak al

        for &eleman2 in dizi2 { // İkinci dizi içerisindeki her bir elemanı eleman2 olarak al
            let fark = (eleman1 - eleman2).abs(); // İki eleman arasındaki farkın mutlak değerini hesapla

            println!("İşlem: |{} - {}| = {}", eleman1, eleman2, fark);
            // Yapılan işlemleri konsola yazdırmak için

            if fark < en_kucuk_deger { // Eğer hesaplanan fark, şu anki en küçük değerden daha küçükse
                en_kucuk_deger = fark; // En küçük değeri güncelle
                println!("Güncellenmiş en küçük fark bulundu: {}", en_kucuk_deger); // Yeni en küçük farkı konsola yazdır
            }
        }
    }

    en_kucuk_deger // Bulunan en küçük farkı döndür
}

fn main() {
    let dizi1 = vec![1, 3, 15, 11, 2]; // İlk dizi tanımlanıyor
    let dizi2 = vec![23, 127, 235, 19, 8]; // İkinci dizi tanımlanıyor

    let sonuc = en_kucuk_fark(&dizi1, &dizi2); // İki dizi arasındaki en küçük farkı hesapla
    println!("İki dizi arasındaki en küçük fark : {}", sonuc); // Sonucu ekrana yazdır
}*/




// Diziyi verilen sayı kadar küçültme fonksiyonu
/*use std::io;

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
}*/




/*use std::io;

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
}*/


/*use std::collections::VecDeque;

// Operatör önceliğini belirleyen fonksiyon
fn oncelik(op: char) -> i32 {
    match op {
        '+' | '-' => 1,  // Toplama ve çıkarma önceliği
        '*' | '/' => 2,  // Çarpma ve bölme önceliği
        _ => 0,          // Diğer karakterler için varsayılan öncelik
    }
}

// İki sayıya bir işlem uygulayan fonksiyon
fn islem_uygula(a: f32, b: f32, op: char) -> f32 {
    match op {
        '+' => a + b,  // Toplama işlemi
        '-' => a - b,  // Çıkarma işlemi
        '*' => a * b,  // Çarpma işlemi
        '/' => a / b,  // Bölme işlemi
        _ => 0.0,      // Geçersiz operatör için varsayılan değer
    }
}

// Şu anki değer ve işlem sıralarını yazdıran yardımcı fonksiyon
fn durum_yazdir(degerler: &VecDeque<f32>, islemler: &VecDeque<char>) {
    println!("Degerler: {:?}", degerler);  // Değer yığını
    println!("Islemler: {:?}", islemler);  // İşlem yığını
    println!("------------------");
}

// Verilen matematiksel ifadeyi değerlendiren ana fonksiyon
fn ifade_degerlendir(ifade: &str) -> f32 {
    let mut degerler: VecDeque<f32> = VecDeque::new();  // Sayıları tutan yığın
    let mut islemler: VecDeque<char> = VecDeque::new(); // İşlem operatörlerini tutan yığın
    let mut i = 0;  // Karakter dizini

    while i < ifade.len() {
        let karakter = ifade.chars().nth(i).unwrap();  // Geçerli karakter

        if karakter == ' ' {  // Boşlukları atla
            i += 1;
            continue;
        }

        // Eğer karakter bir rakam ise, tam sayıyı veya kesirli sayıyı oku
        if karakter.is_digit(10) {
            let mut deger = 0.0;
            while i < ifade.len() && ifade.chars().nth(i).unwrap().is_digit(10) {
                deger = deger * 10.0 + ifade.chars().nth(i).unwrap().to_digit(10).unwrap() as f32;
                i += 1;
            }

            // Ondalık sayıları işleme
            if i < ifade.len() && ifade.chars().nth(i).unwrap() == '.' {
                i += 1;
                let mut kesir = 0.1;
                while i < ifade.len() && ifade.chars().nth(i).unwrap().is_digit(10) {
                    deger += ifade.chars().nth(i).unwrap().to_digit(10).unwrap() as f32 * kesir;
                    kesir *= 0.1;
                    i += 1;
                }
            }

            degerler.push_back(deger);  // Okunan sayıyı yığına ekle
            println!("Sayı eklendi: {}", deger);
            durum_yazdir(&degerler, &islemler);  // Şu anki durumu yazdır
            continue;
        }

        // Eğer karakter açma parantezi ise, operatör yığınına ekle
        if karakter == '(' {
            islemler.push_back(karakter);
            println!("Açma parantezi eklendi");
            durum_yazdir(&degerler, &islemler);
        }

        // Eğer karakter kapanma parantezi ise, parantez içindeki işlemleri uygula
        else if karakter == ')' {
            while !islemler.is_empty() && *islemler.back().unwrap() != '(' {
                let val2 = degerler.pop_back().unwrap();
                let val1 = degerler.pop_back().unwrap();
                let op = islemler.pop_back().unwrap();
                let sonuc = islem_uygula(val1, val2, op);  // İşlemi uygula
                degerler.push_back(sonuc);  // Sonucu yığına ekle
                println!("Parantez içi işlem: {} {} {} = {}", val1, op, val2, sonuc);
                durum_yazdir(&degerler, &islemler);
            }
            islemler.pop_back(); // Açma parantezini kaldır
            println!("Kapanma parantezi işleme alındı");
            durum_yazdir(&degerler, &islemler);
        }

        // Eğer karakter bir operatörse, uygun işlemleri yap ve yığına ekle
        else {
            while !islemler.is_empty() && oncelik(*islemler.back().unwrap()) >= oncelik(karakter) {
                let val2 = degerler.pop_back().unwrap();
                let val1 = degerler.pop_back().unwrap();
                let op = islemler.pop_back().unwrap();
                let sonuc = islem_uygula(val1, val2, op);
                degerler.push_back(sonuc);
                println!("İşlem: {} {} {} = {}", val1, op, val2, sonuc);
                durum_yazdir(&degerler, &islemler);
            }
            islemler.push_back(karakter);
            println!("Operatör eklendi: {}", karakter);
            durum_yazdir(&degerler, &islemler);
        }

        i += 1;  // Sonraki karaktere geç
    }

    // Kalan işlemleri yap
    while !islemler.is_empty() {
        let val2 = degerler.pop_back().unwrap();
        let val1 = degerler.pop_back().unwrap();
        let op = islemler.pop_back().unwrap();
        let sonuc = islem_uygula(val1, val2, op);
        degerler.push_back(sonuc);
        println!("Kalan işlem: {} {} {} = {}", val1, op, val2, sonuc);
        durum_yazdir(&degerler, &islemler);
    }

    degerler.pop_back().unwrap()  // Son sonucu döndür
}

fn main() {
    println!("Sonuç: {}\n", ifade_degerlendir("2*3+5/6*3+15"));  // Ana fonksiyonu çağır ve sonucu yazdır
}*/



// Faktoriyel hesaplayan fonksiyon
/*fn faktoriyel(n: i32) -> i32 {
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
}*/




