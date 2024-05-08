use std::io;

// Сумма двох цифр

/* 1. let mut buff = String::new();: Створюємо порожній рядок для зберігання введення.
2. ::std::io::stdin().read_line(&mut buff);: Читаємо рядок зі стандартного вводу і записуємо його у змінну buff.
3. let mut words = buff.split_whitespace();: Розбиваємо рядок на слова (цифри), використовуючи пробіли як роздільники.
4. let a: i64 = words.next().unwrap().parse().unwrap();: Зчитуємо перше слово (першу цифру), конвертуємо його у число.
5. let b: i64 = words.next().unwrap().parse().unwrap();: Аналогічно зчитуємо друге слово (другу цифру) і конвертуємо його у число.
6. println!("{}", a+b);: Виводимо суму двох чисел.*/

fn main() {
    let mut buff = String::new();
    ::std::io::stdin().read_line(&mut buff);
    let mut words = buff.split_whitespace();

    let a:i64=words.next().unwrap().parse().unwrap();
    let b:i64=words.next().unwrap().parse().unwrap();

    println!("{}", a+b);

}


