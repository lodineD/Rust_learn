#![allow(dead_code)]

use std::fmt;

// 1. 一个包含两个字段的结构体
struct Point {
    x: i32,
    y: i32,
}

// 2. 一个包含 Vec 的结构体
struct ShoppingList {
    items: Vec<String>,
}

// 3. 一个枚举
enum Color {
    Red,
    Green,
    Blue,
}

// 给 Point 实现 Display
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// 给 ShoppingList 实现 Display
impl fmt::Display for ShoppingList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "购物清单: [{}]", self.items.join(", "))
    }
}

// 给 Color 实现 Display
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Color::Red => "红色",
            Color::Green => "绿色",
            Color::Blue => "蓝色",
        };
        write!(f, "{}", name)
    }
}

fn main() {
    // 同一个 println!("{}", ...) 语法，却能打印不同类型！
    let p = Point { x: 3, y: 4 };
    println!("Point: {}", p);

    let list = ShoppingList { items: vec!["苹果".into(), "牛奶".into()] };
    println!("{}", list);

    let c = Color::Green;
    println!("颜色: {}", c);
}