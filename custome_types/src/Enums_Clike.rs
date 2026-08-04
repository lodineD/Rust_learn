// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);// 无数据可以自然的转整数，从0开始数
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as u32);// 已经被赋值了
    println!("violets are #{:06x}", Color::Blue as u32);
}