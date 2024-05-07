use std::io;


// Сумма двох цифр4

fn main() {
    let mut a = String::new();
    let mut b = String::new();

    println!("Введіть цифру #1 ");
    io::stdin().read_line(&mut a).expect("Не вдалося прочитати рядок");
    let dijit_1:i32=a.trim().parse().expect("Введено невірне число");

    println!("Введіть цифру #2 ");
    io::stdin().read_line(&mut b).expect("Не вдалося прочитати рядок");
    let dijit_2:i32=b.trim().parse().expect("Введено невірне число");


    let sum = dijit_1 + dijit_2;

    println!("Sum: {}", sum);
}
