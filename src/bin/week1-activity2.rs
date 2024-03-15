// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Buat sebuah fungsi untuk menambahkan dua angka
// * Buat sebuah fungsi untuk menampilkan hasilnya * Gunakan token "{:?}" pada macro println! untuk menampilkan hasilnya

fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}

fn main() {
    let a = 10;
    let b = 20;
    println!("{:?}", add_two_numbers(a, b));
}
