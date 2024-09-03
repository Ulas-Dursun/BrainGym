
fn en_kucuk_fark(dizi1: &[i32], dizi2: &[i32]) -> i32 {
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

    let sonuc = crate::en_kucuk_fark(&dizi1, &dizi2); // İki dizi arasındaki en küçük farkı hesapla
    println!("İki dizi arasındaki en küçük fark : {}", sonuc); // Sonucu ekrana yazdır
}