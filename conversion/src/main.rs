use std::convert::From;
use std::fmt;

#[derive(Debug)]
struct Number {
    value: i32,
}

// From<i32>本质上也是一个trait，也就是说，让Number，使用from时转换成{value},本身println是不支持的
// 当你实现了from后,into也就自动实现了,
// 不是所有的from都实现了,实际上from是一个标准库,在string中,已经实现好了,使用from可以直接实现String类,但是自定义的情况是没有的
// 因此需要自行定义from
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "这是一个全新的struct: {}", self.value)
    }
}


fn main() {
    let num = Number::from(30);
    println!("{}", num);

    let int = 5;
    // 尝试移除类型标注
    let num: Number = int.into();
    println!("{}", num);
}