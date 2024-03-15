// Topic: Control Flow
//
// Program requirements:
// * Menampilkan Fizz jika angka tersebut habis dibagi 3
// * Menampilkan Buzz jika angka tersebut habis dibagi 5
// * Menampilkan FizzBuzz jika angka tersebut habis dibagi 3 dan 5
//
// Notes:
// * Buatlah sebuah fungsi yang menerima parameter angka
// * Gunakan control flow berupa if else if else untuk mengecek kelipatan 3 dan 5
// * Panggil fungsi tersebut di dalam fungsi main
// * Loop dari 1 sampai 100 untuk memanggil fungsi tersebut
// * Gunakan token "{:?}" pada macro println! untuk menampilkan hasilnya

fn fizz_buzz(num: i32) {
    if num % 3 == 0 && num % 5 == 0 {
        println!("FizzBuzz");
    } else if num % 3 == 0 {
        println!("Fizz");
    } else if num % 5 == 0 {
        println!("Buzz");
    }
}

fn main() {
    let mut i = 1;
    while i <= 100 {
        fizz_buzz(i);
        i += 1;
    }
}
