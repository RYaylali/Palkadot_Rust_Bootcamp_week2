use std::io;

enum Operation {
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}
fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a + b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}

fn main() {
    println!("Enter the first number:"); // Kullanıcıya ilk sayıyı girer
    let mut input = String::new(); //kullanıcı girişini saklamak için ifade oluşturuyoruz
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input"); //Kullanıcı veri giriyor
    let first_number: f64 = input.trim().parse().expect("Invalid input"); //girilen sayının etrafını temizleyip sayıya çeviriyoruz ve değişkene kayıt ediyoruz

    println!("Enter the operation (+, -, *, /):"); // İşlemi seçmesi için kullanıcıya işlem türünü girmesini söylüyoruz.
    input.clear(); //Önceki sayı temizlenir
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input"); //Kullanıcı girişi okuma
    let operation = match input.trim() {
        "+" => Operation::Add,
        "-" => Operation::Subtract,
        "*" => Operation::Multiply,
        "/" => Operation::Divide,
        _ => {
            println!("Invalid operation");
            return;
        }
    }; //Kişinin seçtiği işlem operatü belirlenir

    println!("Enter the second number:"); //İkinci sayı girilmesi istenir
    input.clear(); //veri almak için kullanılan input temizlenirs
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input"); //inputa ikinci değer kullanıcı tarafından atanır
    let second_number: f64 = input.trim().parse().expect("Invalid input"); //atanan ikinci değer başka değişkene atanır

    let operation_enum = operation(first_number, second_number); //seçilen işlem trüne göre örneğin add(a,b ) tarzında enumda değerler oluşturulur
    let result = calculate(operation_enum); //calculation metodu ile işlme yapılır

    println!("Result: {}", result); //sonuç yazdırılır
}
